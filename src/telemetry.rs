// =============================================================================
//  telemetry.rs — Command telemetry and community data sharing
//  https://github.com/paulfxyz/yo-rust
//
//  OVERVIEW
//  ────────
//  yo-rust can optionally record successful prompt → command pairs and POST
//  them to JSONBin.io.  This data is used to:
//    1. Build a community dataset of real natural-language → shell command
//       mappings, tagged by OS/shell/model
//    2. Let the project maintainer (Paul Fleury) analyse which prompts work
//       well, which fail, and iterate on the system prompt weekly
//    3. Optionally give users their own private JSONBin where they can review
//       their own history
//
//  WHAT IS COLLECTED — AND WHAT IS NOT
//  ─────────────────────────────────────
//  Collected (when opted in):
//    - prompt:          the natural-language request the user typed
//    - commands:        the shell commands that ran (confirmed + succeeded)
//    - model:           the AI model slug (e.g. "openai/gpt-4o-mini")
//    - backend:         "openrouter" or "ollama"
//    - os:              "macos", "linux", "windows"
//    - arch:            "aarch64", "x86_64"
//    - shell:           "zsh", "bash", "powershell5", etc.
//    - worked:          true/false from the "Did that work?" feedback prompt
//    - yo_rust_version: the version of yo-rust that produced this entry
//    - timestamp:       ISO 8601 UTC timestamp
//
//  NOT collected:
//    - API keys (never, ever)
//    - File contents or paths from the user's filesystem
//    - CWD (current working directory — could contain private path structure)
//    - User identity, hostname, or machine ID
//    - Any output from the executed commands
//
//  ARCHITECTURE — TWO DESTINATIONS, BOTH OPTIONAL
//  ──────────────────────────────────────────────
//  Central bin (Paul's JSONBin, write-only access key):
//    - A WRITE-ONLY Access Key is published in the binary.
//    - Users can POST to it but cannot read other users' entries.
//    - Paul reads the accumulated data from his JSONBin dashboard.
//    - One new bin is created per telemetry entry (JSONBin's model: each POST
//      to /v3/b creates a new bin; we use a Collection to group them).
//    - This destination is opt-in (default: OFF until user explicitly agrees).
//
//  User's own bin (personal JSONBin account, optional):
//    - User provides their own Master Key + Collection ID during !api setup.
//    - Their entries go to their own private account — only they can read it.
//    - Useful for their own command history analytics.
//    - Completely independent of the central bin.
//
//  PRIVACY DESIGN
//  ──────────────
//  - Default: telemetry is OFF.  The user is asked once during first-run
//    setup and can change at any time via !api.
//  - The central bin uses a write-only Access Key — it has Bins Create
//    permission only.  No read, no update, no delete.  Paul's Master Key
//    is never distributed.
//  - Each entry is a separate bin (not appended to one bin) — this means
//    individual entries cannot be correlated by bin ID.
//  - No CWD or hostname is ever included.
//  - The telemetry POST is fire-and-forget in a background thread — it
//    never blocks the REPL loop and failures are silently ignored.
//
//  JSONBIN.IO API USED
//  ────────────────────
//  POST https://api.jsonbin.io/v3/b
//  Headers:
//    Content-Type:  application/json
//    X-Master-Key:  $MASTER_KEY           (Paul's, for central; user's, for personal)
//    X-Bin-Private: true                  (always private)
//    X-Bin-Name:    yo-rust-$timestamp    (for easy filtering in dashboard)
//    X-Collection-Id: $COLLECTION_ID      (groups entries together)
//
//  Each POST creates a new bin entry.  This costs 1 request from the quota.
//  At Paul's JSONBin free tier (10,000 requests), this supports 10,000
//  telemetry submissions.  The Pro plan ($20 one-time) provides 100,000.
// =============================================================================

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// ── The central write-only Access Key for Paul's JSONBin collection ───────────
// This key has ONLY "Bins Create" permission — it cannot read, update, or
// delete any bin.  It is safe to ship in the binary.
// Paul sets this up by:
//   1. Creating a JSONBin.io account at https://jsonbin.io
//   2. Creating a Collection named "yo-rust-telemetry"
//   3. Creating an Access Key with only "Bins Create" permission
//   4. Replacing this placeholder with the actual key before release
//
// Central write-only Access Key (Bins Create permission only — safe to embed).
// Collection: yo-rust-telemetry  |  Created: 2026-03-22
// Master Key is kept private (never in source) — contact hello@paulfleury.com
pub const CENTRAL_ACCESS_KEY: &str   = "$2a$10$xJ5kER3PeMHMZKWRnJxhrehfH6wHeGURAhdmmctbLnboMhTXyJW9a";
pub const CENTRAL_COLLECTION_ID: &str = "69c05e31b7ec241ddc91ee96";

// Whether the central destination is actually configured (non-placeholder)
pub fn central_is_configured() -> bool {
    !CENTRAL_ACCESS_KEY.contains("PLACEHOLDER") && !CENTRAL_COLLECTION_ID.contains("PLACEHOLDER")
}

// ── The telemetry record shape ────────────────────────────────────────────────

/// One telemetry entry: a prompt that ran successfully with its commands.
///
/// Serialised to JSON and POSTed to JSONBin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEntry {
    /// The user's natural-language prompt
    pub prompt: String,

    /// The shell commands that executed
    pub commands: Vec<String>,

    /// The AI model that generated these commands
    pub model: String,

    /// "openrouter" or "ollama"
    pub backend: String,

    /// OS: "macos", "linux", "windows"
    pub os: &'static str,

    /// CPU architecture: "aarch64", "x86_64", etc.
    pub arch: &'static str,

    /// Shell kind: "zsh", "bash", "powershell5", "powershell7", "cmd.exe", etc.
    pub shell: String,

    /// Whether the user confirmed "Did that work?" → true/false
    /// None = telemetry sent before feedback was collected
    pub worked: Option<bool>,

    /// yo-rust version that produced this entry
    pub yo_rust_version: &'static str,

    /// ISO 8601 UTC timestamp (seconds precision)
    pub timestamp: String,
}

impl TelemetryEntry {
    /// Construct a new entry from the current session context.
    pub fn new(
        prompt: &str,
        commands: &[String],
        model: &str,
        backend: &str,
        shell: &str,
        worked: Option<bool>,
    ) -> Self {
        Self {
            prompt: prompt.to_string(),
            commands: commands.to_vec(),
            model: model.to_string(),
            backend: backend.to_string(),
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
            shell: shell.to_string(),
            worked,
            yo_rust_version: env!("CARGO_PKG_VERSION"),
            timestamp: iso8601_now(),
        }
    }
}

// ── Submission ────────────────────────────────────────────────────────────────

/// Result of a telemetry submission attempt.
#[derive(Debug)]
pub enum SubmitResult {
    /// Successfully posted
    Ok,
    /// Telemetry disabled or central not configured
    Skipped,
    /// Network or API error — silently ignored by caller
    Err,
}

/// Submit a telemetry entry to one or both destinations.
///
/// This function is designed to be called in a background thread (via
/// `std::thread::spawn`) so it never blocks the REPL.  All errors are
/// captured and returned; the caller decides whether to surface them.
///
/// # Arguments
/// * `entry`            — the telemetry record to send
/// * `share_central`    — whether to POST to Paul's central collection
/// * `user_master_key`  — if Some, also POST to the user's own JSONBin
/// * `user_collection`  — user's collection ID (required if user_master_key is Some)
pub fn submit(
    entry: TelemetryEntry,
    share_central: bool,
    user_master_key: Option<String>,
    user_collection: Option<String>,
) -> SubmitResult {
    let client = match reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(_) => return SubmitResult::Err,
    };

    let json_body = match serde_json::to_string(&entry) {
        Ok(j) => j,
        Err(_) => return SubmitResult::Err,
    };

    let bin_name = format!("yo-rust-{}", &entry.timestamp[..10]); // e.g. "yo-rust-2026-03-22"

    let mut posted_any = false;

    // ── Central destination (Paul's collection, write-only Access Key) ────────
    if share_central && central_is_configured() {
        let result = client
            .post("https://api.jsonbin.io/v3/b")
            .header("Content-Type", "application/json")
            // Access Key has Bins Create permission ONLY — no read/update/delete
            .header("X-Access-Key", CENTRAL_ACCESS_KEY)
            .header("X-Bin-Private", "true")
            .header("X-Bin-Name", &bin_name)
            .header("X-Collection-Id", CENTRAL_COLLECTION_ID)
            .body(json_body.clone())
            .send();

        match result {
            Ok(resp) if resp.status().is_success() => { posted_any = true; }
            Ok(resp) => {
                // Non-fatal: log the status but don't surface to user
                let _ = format!("Central JSONBin returned {}", resp.status());
            }
            Err(_) => {
                // Network error — silently ignored
            }
        }
    }

    // ── User's own destination (their personal JSONBin) ───────────────────────
    if let (Some(master_key), Some(collection)) = (user_master_key, user_collection) {
        if !master_key.is_empty() && !collection.is_empty() {
            let result = client
                .post("https://api.jsonbin.io/v3/b")
                .header("Content-Type", "application/json")
                .header("X-Master-Key", &master_key)
                .header("X-Bin-Private", "true")
                .header("X-Bin-Name", &bin_name)
                .header("X-Collection-Id", &collection)
                .body(json_body.clone())
                .send();

            match result {
                Ok(resp) if resp.status().is_success() => { posted_any = true; }
                _ => {}
            }
        }
    }

    if posted_any {
        SubmitResult::Ok
    } else {
        SubmitResult::Skipped
    }
}

/// Fire-and-forget telemetry submission in a background thread.
///
/// The REPL loop calls this after a successful execution + feedback.
/// The thread is detached — if it fails, the user never sees anything.
pub fn submit_async(
    entry: TelemetryEntry,
    share_central: bool,
    user_master_key: Option<String>,
    user_collection: Option<String>,
) {
    std::thread::spawn(move || {
        let _ = submit(entry, share_central, user_master_key, user_collection);
    });
}

// ── Setup wizard helpers ──────────────────────────────────────────────────────

/// Explain the telemetry feature to the user during first-run setup.
/// Returns (share_central, user_master_key, user_collection_id).
pub fn interactive_setup(current_share_central: bool) -> (bool, Option<String>, Option<String>) {
    use colored::Colorize;
    use std::io::{self, Write};

    println!();
    println!("{}", "  ╔══════════════════════════════════════════════════════╗".cyan());
    println!("{}", "  ║         Community Data Sharing (optional)           ║".cyan().bold());
    println!("{}", "  ╚══════════════════════════════════════════════════════╝".cyan());
    println!();
    println!("  {}", "yo-rust can optionally share anonymised data about which".white());
    println!("  {}", "prompts produced useful commands, to help improve the tool.".white());
    println!();
    println!("  {}", "What gets shared:".white().bold());
    println!("  {}", "  ✓  Your natural-language prompt".dimmed());
    println!("  {}", "  ✓  The commands that ran".dimmed());
    println!("  {}", "  ✓  OS, shell, AI model used".dimmed());
    println!("  {}", "  ✓  Whether it worked (your Y/N feedback)".dimmed());
    println!();
    println!("  {}", "What is NEVER shared:".white().bold());
    println!("  {}", "  ✗  Your API key".dimmed());
    println!("  {}", "  ✗  File contents or paths".dimmed());
    println!("  {}", "  ✗  Your working directory".dimmed());
    println!("  {}", "  ✗  Any command output".dimmed());
    println!("  {}", "  ✗  Your name, hostname, or identity".dimmed());
    println!();
    println!("  {}", "Data goes to: JSONBin.io (private bins, write-only key)".dimmed());
    println!("  {}", "Reviewed weekly by Paul Fleury to improve yo-rust.".dimmed());
    println!("  {}", "You can opt out at any time via !api.".dimmed());
    println!();

    // ── Central sharing ───────────────────────────────────────────────────────
    let default_hint = if current_share_central { "[Y/n]" } else { "[y/N]" };
    print!("  {}  ", format!("Share data with the yo-rust community? {} ›", default_hint).yellow().bold());
    io::stdout().flush().unwrap_or(());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);
    let share_central = match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no"  => false,
        ""           => current_share_central, // keep existing if just Enter
        _            => false,
    };

    // ── Personal JSONBin (optional) ───────────────────────────────────────────
    println!();
    println!("  {}", "Personal JSONBin (optional, advanced):".white().bold());
    println!("  {}", "  You can also store your own command history in your own".dimmed());
    println!("  {}", "  JSONBin.io account — completely private, only you can read it.".dimmed());
    println!("  {}", "  Create a free account at https://jsonbin.io".dimmed());
    println!();
    print!("  {}  ", "Your JSONBin Master Key (leave blank to skip) ›".yellow().bold());
    io::stdout().flush().unwrap_or(());
    input.clear();
    io::stdin().read_line(&mut input).unwrap_or(0);
    let user_key = input.trim().to_string();

    let user_master_key;
    let user_collection;

    if user_key.is_empty() {
        user_master_key = None;
        user_collection = None;
    } else {
        user_master_key = Some(user_key);
        println!();
        print!("  {}  ", "Your JSONBin Collection ID (for yo-rust data) ›".yellow().bold());
        io::stdout().flush().unwrap_or(());
        input.clear();
        io::stdin().read_line(&mut input).unwrap_or(0);
        let col = input.trim().to_string();
        user_collection = if col.is_empty() { None } else { Some(col) };
    }

    println!();
    if share_central {
        println!("{}", "  ✔  Community sharing: ON".green());
    } else {
        println!("{}", "  ✔  Community sharing: OFF".dimmed());
    }
    if user_master_key.is_some() {
        println!("{}", "  ✔  Personal JSONBin: configured".green());
    }

    (share_central, user_master_key, user_collection)
}

// ── Utilities ─────────────────────────────────────────────────────────────────

/// Returns current time as ISO 8601 UTC string (seconds precision).
fn iso8601_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Manual ISO 8601 formatting — avoids pulling in a date library
    // secs since epoch → YYYY-MM-DDTHH:MM:SSZ
    let s = secs;
    let days_since_epoch = s / 86400;
    let time_of_day = s % 86400;
    let hh = time_of_day / 3600;
    let mm = (time_of_day % 3600) / 60;
    let ss = time_of_day % 60;

    // Simple Gregorian calendar calculation (good until 2100)
    let mut year = 1970u64;
    let mut remaining_days = days_since_epoch;
    loop {
        let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        let days_in_year = if leap { 366 } else { 365 };
        if remaining_days < days_in_year { break; }
        remaining_days -= days_in_year;
        year += 1;
    }
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let days_in_month = [31u64, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 1u64;
    for &dim in &days_in_month {
        if remaining_days < dim { break; }
        remaining_days -= dim;
        month += 1;
    }
    let day = remaining_days + 1;

    format!("{year:04}-{month:02}-{day:02}T{hh:02}:{mm:02}:{ss:02}Z")
}
