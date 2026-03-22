# 🤖 Yo, Rust!

<div align="center">

**Natural language → Terminal commands, powered by AI.**

*Type `yo` and talk to your terminal like a human being.*

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Powered by OpenRouter](https://img.shields.io/badge/Powered%20by-OpenRouter-6c47ff?style=for-the-badge)](https://openrouter.ai)
[![Ollama](https://img.shields.io/badge/Supports-Ollama-black?style=for-the-badge)](https://ollama.ai)
[![Version](https://img.shields.io/badge/Version-2.3.3-brightgreen?style=for-the-badge)](CHANGELOG.md)
[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-blue?style=for-the-badge)]()
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=for-the-badge)](https://github.com/paulfxyz/yo-rust/pulls)

</div>

---

```
  ╔══════════════════════════════════════════════════════════════════╗
  ║           ╷▲╷             ██╗   ██╗ ██████╗                     ║
  ║      ┌────┴─┴────┐        ╚██╗ ██╔╝██╔═══██╗                    ║
  ║      │ ╔═══╗╔═══╗│         ╚████╔╝ ██║   ██║                    ║
  ║      │ ║◈  ◈║◈  ◈║│          ╚██╔╝  ██║   ██║                    ║
  ║      │ ╚═══╝╚═══╝│           ██║   ╚██████╔╝                    ║
  ║ ┌──┐ │ ┌─────────┐ │ ┌──┐    ╚═╝    ╚═════╝                     ║
  ║ │░░├─┤ │ · · · · │ ├─░░│                                        ║
  ║ └──┘ │ ┌──┬──┬──┐ │ └──┘   ██████╗ ██╗   ██╗███████╗████████╗  ║
  ║      │ │▓▓│▓▓│▓▓│ │        ██╔══██╗██║   ██║██╔════╝╚══██╔══╝  ║
  ║      │ └──┴──┴──┘ │        ██████╔╝██║   ██║███████╗   ██║     ║
  ║      └─────┬─┬────┘        ██╔══██╗██║   ██║╚════██║   ██║     ║
  ║            │ │             ██║  ██║╚██████╔╝███████║   ██║     ║
  ║           ┌┘ └┐            ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝     ║
  ║          │░░││░░│           v2.3.3  ·  github.com/paulfxyz       ║
  ╚══════════════════════════════════════════════════════════════════╝
```

---

## 👨‍💻 Why this exists

I'm **Paul Fleury** — French internet entrepreneur based in Lisbon, managing infrastructure across multiple products: DNS records, Docker stacks, reverse proxies, SSL certificates, CI/CD pipelines, API integrations, database backups, cron jobs. The whole sysadmin surface.

I kept hitting the same wall — not the hard problems, but the *tedious* ones.

The `find` command with the exact combination of flags I need to delete files older than 7 days. The `rsync` invocation that syncs without accidentally wiping the destination. The `awk` one-liner to extract column 3 from a log file. The `openssl` command that decodes a PEM certificate. The `lsof` incantation to kill whatever is on port 3000. Things I've typed dozens of times over the years but never fully memorised, because I don't type them *every single day*.

The ritual was always the same: stop what I'm doing → open a browser tab → Google the thing → skip ads → skim Stack Overflow → find an answer from 2016 that might be outdated → adapt it → test it → close the tab. **Five minutes gone.** Times ten per day. That's an hour of command-syntax archaeology every day.

I wanted something that felt like messaging a friend who knows Linux perfectly. You just describe what you want. You get the command. You run it.

The key constraints I set for myself from day one:

- **No runtime.** I've been burned by Python version conflicts, Node.js dependency hell, and Ruby gem incompatibilities too many times. A tool that requires "pip install" or "npm install" before you can use it is not a tool — it's a project. I wanted a single compiled binary that works on any machine, forever, without installation ceremonies.
- **One command to install.** `curl | bash`. Done. Even Rust itself should install automatically.
- **Any AI model.** Not locked to one provider. OpenRouter gives access to GPT-4, Claude, Llama, and dozens of others behind one key. For air-gapped or privacy-sensitive environments, Ollama runs local models with no network access at all.
- **Safe by default.** Nothing executes without explicit confirmation. The user always sees what will run before it runs.

The answer to all these constraints was **Rust**. This document explains why.

> 💡 This project was designed and built in collaboration with **[Perplexity Computer](https://www.perplexity.ai)** — from architecture through implementation, debugging, and documentation. A genuine example of human intent meeting AI execution.

---

## 🦀 Why Rust — a serious answer

The choice of Rust for this project was deliberate, not fashionable. Here's the real reasoning.

### The binary size argument

A Python CLI tool requires Python to be installed, the right version, with the right packages, in the right virtualenv. A Node.js tool requires Node, npm, and potentially hundreds of packages in `node_modules`. These are not academic concerns — in production they cause real failures. The wrong Python version, a missing package, a broken lockfile.

A compiled Rust binary is a single self-contained executable. Copy it to any machine running the same OS and architecture — it works. No interpreter, no runtime, no dependencies. `yo.exe` on Windows, `yo` on macOS or Linux. That's it.

For a tool you want to install once and trust forever, this matters enormously.

### The performance argument

Rust compiles to native machine code via LLVM, the same optimisation infrastructure used by C and C++. The result is performance comparable to C with a modern ergonomic language on top.

For yo-rust, the bottleneck is the network call to OpenRouter — typically 500ms to 3 seconds. The Rust binary itself starts in under 10 milliseconds. The terminal UI renders instantly. There is no "warming up the interpreter" moment, no garbage collection pause, no JIT compilation lag.

This is not academic. When you type `yo` you want to see the prompt immediately, not wait 500ms for Python to import its modules.

### The memory safety argument

Rust's ownership and borrowing system enforces memory safety at compile time, without a garbage collector. This eliminates entire categories of bugs:
- No buffer overflows
- No use-after-free
- No null pointer dereferences
- No data races between threads

For yo-rust, this is directly relevant. We spawn background threads for telemetry submissions. In C or C++, passing data between threads while the main thread continues is a minefield. In Rust, the compiler refuses to compile code that would create a data race. The background telemetry thread's data is moved, not shared — the compiler enforces this.

### The correctness argument

Rust's type system is expressive enough to encode many invariants at compile time:
- `Option<T>` forces you to handle the "not present" case explicitly — no null pointer exceptions
- `Result<T, E>` forces you to handle errors explicitly — no uncaught exceptions
- Exhaustive pattern matching means if you add a new variant to an enum, the compiler tells you everywhere you forgot to handle it

In yo-rust, the `ShellKind` enum has variants for every shell type. If a future version adds a new shell, the Rust compiler will produce an error at every match expression that doesn't handle the new variant. In Python or JavaScript, you'd ship a bug and discover it at runtime.

### The `cargo` build system

Rust's package manager, `cargo`, is one of the best build tools in existence:
- `cargo build --release` produces an optimised binary
- `cargo check` type-checks the entire project in seconds without producing a binary
- `Cargo.toml` is a clean, readable manifest with exact version locking via `Cargo.lock`
- Cross-compilation to other platforms is built in

The entire yo-rust project builds with a single command. No Makefile, no CMakeLists.txt, no build.gradle. Just `cargo build --release`.

### The production-grade standard library

Rust's standard library includes production-quality implementations of:
- Threading (`std::thread`) — used for background telemetry
- File I/O (`std::fs`) — used for config and history files
- Environment variables (`std::env`) — used for shell detection
- Process spawning (`std::process::Command`) — used for command execution
- Time (`std::time`) — used for ISO 8601 timestamps

No third-party dependencies needed for any of these. The standard library is battle-tested and stable across platforms including Windows.

### The ecosystem

The Rust ecosystem (crates.io) has high-quality crates for everything yo-rust needs:
- `reqwest` — HTTP client used by half the Rust ecosystem
- `serde` + `serde_json` — the standard JSON library, zero-cost abstractions
- `clap` — the standard CLI argument parser, used in major Rust projects
- `rustyline` — readline-style line editing, same ergonomics as bash history
- `colored` — ANSI terminal colours with NO_COLOR support

These are all mature, well-maintained libraries with stable APIs.

### Rust is not just for systems programming

There's a misconception that Rust is only for operating systems, game engines, and embedded firmware. Yo-rust demonstrates that Rust is excellent for command-line tools and productivity applications.

The compile times are longer than Python (though improving rapidly). The learning curve is steeper. But the result is a binary that ships, works everywhere, starts instantly, and never crashes due to memory errors or type errors. For a tool that people will install and trust to run shell commands, that's the right trade-off.

---

## 🌟 Feature overview

| Feature | Detail |
|---|---|
| 🗣️ Natural language | Plain English → shell commands via any OpenRouter model or local Ollama |
| ✅ Always confirms | Every suggestion requires `Y` before anything runs — safe by design |
| ⚡ Single binary | No Python, Node.js, or runtime — one file, works everywhere |
| 🔑 Local config | API key stored in your OS config directory, never transmitted elsewhere |
| 🤖 ASCII banner | Full robot illustration + block-letter logo on every launch |
| 🧠 Intent detection | "use ollama" / "change model" triggers reconfiguration without an API call |
| 📟 Rich shortcuts | `!help`, `!api`, `!context`, `!clear`, `!feedback`, `!shortcuts`, `!exit` |
| 🐚 Shell aliases | `yo`, `hi`, or `hello` — all three work |
| 🌍 Context-aware | OS, arch, CWD, and precise shell sent with every request |
| 🛡️ Safe prompting | Temperature 0.2 — deterministic, conservative command suggestions |
| 💬 Explanations | Every suggestion includes a one-sentence plain-English description |
| 📜 Session history | ↑/↓ keys recall previous prompts within a session |
| 🏠 Ollama support | Local AI inference — no API key, no internet, complete privacy |
| 🔁 Multi-turn context | Follow-up prompts resolve correctly ("now do the same for X") |
| 📂 Shell history | Confirmed commands appended to `~/.zsh_history` / `~/.bash_history` |
| 🧪 Dry-run | `yo --dry` — see every command before any execution |
| 🪝 Feedback loop | "Did that work?" with AI refinement if it didn't |
| 🪟 Windows native | PS5, PS7, cmd.exe, Git Bash — all detected, correct syntax generated |
| 💾 Named shortcuts | `!save`, `!forget`, replay any command set instantly with `!<name>` |
| 📊 Telemetry | Opt-in community data sharing via JSONBin.io to improve the tool |

---

## 🚀 Install

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/paulfxyz/yo-rust/main/yo.sh | bash
```

**Windows — PowerShell** (native, no Git Bash needed):
```powershell
iwr -useb https://raw.githubusercontent.com/paulfxyz/yo-rust/main/install.ps1 | iex
```

**Windows — Git Bash or WSL2:**
```bash
curl -fsSL https://raw.githubusercontent.com/paulfxyz/yo-rust/main/yo.sh | bash
```

> ⚠️ On Windows, `curl` in PowerShell is an alias for `Invoke-WebRequest` and does **not** accept `-fsSL` flags. Always use the `iwr` command or open Git Bash.

**Update:**
```bash
curl -fsSL https://raw.githubusercontent.com/paulfxyz/yo-rust/main/update.sh | bash      # macOS/Linux
iwr -useb https://raw.githubusercontent.com/paulfxyz/yo-rust/main/update.ps1 | iex        # Windows PS
```

**Uninstall:**
```bash
curl -fsSL https://raw.githubusercontent.com/paulfxyz/yo-rust/main/uninstall.sh | bash    # macOS/Linux
iwr -useb https://raw.githubusercontent.com/paulfxyz/yo-rust/main/uninstall.ps1 | iex     # Windows PS
```

> Full guide: **[INSTALL.md](INSTALL.md)**

---

## 🎬 See it in action

```
$ yo

  [banner]

  ◈  Backend: OpenRouter  model: openai/gpt-4o-mini
  ◈  Shell history: on
  ◈  Context: 5 turns

  yo ›  find all log files older than 7 days and delete them

  ◈  Finds .log files not modified in 7+ days and removes them.

  ┌────────────────────────────────────────────────────────┐
  │  $  find . -name "*.log" -mtime +7 -type f -delete    │
  └────────────────────────────────────────────────────────┘

  Run it? [Y/n] › Y

  ►  find . -name "*.log" -mtime +7 -type f -delete
  ✔  Done.

  Did that work? [Y/n] › Y
  ◈  Great! What else?

  yo [+1] ›  now do the same for the /tmp folder

  ◌  Thinking…

  ◈  Applies the same log cleanup to /tmp.

  ┌────────────────────────────────────────────────────────────┐
  │  $  find /tmp -name "*.log" -mtime +7 -type f -delete     │
  └────────────────────────────────────────────────────────────┘

  Run it? [Y/n] ›
```

The `[+1]` shows how many prior turns the AI has in its context window. "now do the same for" works because the previous prompt and its commands were injected as prior conversation turns.

**More examples:**

```
yo ›  kill whatever is running on port 8080
  ◈  Finds and terminates the process listening on port 8080.
  $ lsof -ti:8080 | xargs kill -9

yo ›  show the 10 biggest files in this folder
  ◈  Lists files sorted by size, showing the 10 largest.
  $ du -ah . | sort -rh | head -n 10

yo ›  watch the nginx error log live
  ◈  Streams new lines from the nginx error log as they appear.
  $ tail -f /var/log/nginx/error.log

yo ›  check my public IP address
  ◈  Queries a public API and prints your external IP.
  $ curl -s https://ifconfig.me

yo ›  git log for last 7 days with author names
  ◈  Shows commits from the past week with author and subject.
  $ git log --since="7 days ago" --pretty=format:"%h %an %ad: %s" --date=short
```

---

## 🏠 Ollama — local, private, offline

Run yo-rust entirely on your own machine with no API key and no network access:

```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh   # macOS / Linux
# Windows: download from https://ollama.ai/download

# Pull a model (one-time download)
ollama pull llama3.2        # 2GB, recommended general-purpose
ollama pull mistral         # 4GB, fast, good at commands
ollama pull codellama       # code-optimised

# Launch yo-rust — choose Ollama during setup
yo
```

Or switch from within a session at any time:
```
yo ›  use ollama
```

Ollama runs a local HTTP server at `http://localhost:11434`. yo-rust talks to it using the same chat completion API format as OpenRouter, so the experience is identical — just no API key, no billing, and no data leaving your machine.

### When to use Ollama vs OpenRouter

| Situation | Recommendation |
|---|---|
| Air-gapped or restricted environment | Ollama |
| Privacy-sensitive commands | Ollama |
| Development / offline work | Ollama |
| Best accuracy, complex requests | OpenRouter (GPT-4o or Claude) |
| Fast, cheap, reliable daily use | OpenRouter (gpt-4o-mini) |

---

## 🧪 Dry-run mode

Preview exactly what yo-rust would do, with zero side effects:

```bash
yo --dry
```

The AI is called normally, suggestions are displayed in a **yellow** command box with `[dry-run — not executed]`, but nothing ever runs. Context is still updated, so follow-up prompts work. Shell history is not appended.

Use this before any destructive operation, for demos, or when you want to see what a complex prompt would generate before committing.

---

## 🐚 Shell detection — precise, cross-platform

yo-rust detects your exact shell at launch and passes it to the AI as `SHELL=<kind> syntax=<family>`. This single addition has the highest impact on command correctness of anything in the codebase.

| Shell | Detected as | AI generates |
|---|---|---|
| zsh | `SHELL=zsh syntax=posix` | POSIX + zsh idioms |
| bash | `SHELL=bash syntax=posix` | POSIX + bash idioms |
| fish | `SHELL=fish syntax=posix` | fish-compatible syntax |
| sh / dash | `SHELL=sh syntax=posix` | Pure POSIX, no bashisms |
| PowerShell 5 | `SHELL=powershell5 syntax=powershell` | PS5-safe (uses `;` not `&&`) |
| PowerShell 7 | `SHELL=powershell7 syntax=powershell` | PS7 (supports `&&`) |
| cmd.exe | `SHELL=cmd.exe syntax=cmd` | `&` chaining, `%VARIABLE%` |
| Git Bash | `SHELL=gitbash syntax=posix` | POSIX via MSYS2 on Windows |

Detection uses environment variables: `$SHELL` on Unix, `$PSModulePath` version strings to distinguish PS5 from PS7, `$MSYSTEM` to identify Git Bash, `$COMSPEC` for cmd.exe. See `src/shell.rs` for the full detection matrix.

---

## 💾 Named command shortcuts

Save any command set as a named shortcut and replay it instantly — no AI call, no confirmation:

```
yo ›  docker restart myapp && docker logs --tail 50 myapp
  ✔  Done.
  Did that work? [Y/n] › Y

yo ›  !save restartapp

# Any time later:
yo ›  !restartapp
  ◈  Running shortcut !restartapp
  ►  docker restart myapp && docker logs --tail 50 myapp
  ✔  Done.
```

| Command | What it does |
|---|---|
| `!save <name>` | Save last confirmed commands as `!<name>` |
| `!<name>` | Run a shortcut instantly — no AI, no Y/N |
| `!forget <name>` | Remove a shortcut |
| `!shortcuts` / `!sc` | List all saved shortcuts |

Shortcuts are persisted to `~/.config/yo-rust/shortcuts.json` and survive across sessions.

---

## ⌨️ All shortcuts

| Input | What happens |
|---|---|
| `!help` / `!h` | Full help screen with examples and session status |
| `!api` | Reconfigure backend, model, API key, history, context |
| `!feedback` / `!fb` | Telemetry status, opt-in/out, personal JSONBin |
| `!context` / `!ctx` | Show what the AI currently remembers |
| `!clear` | Clear conversation context |
| `!shortcuts` / `!sc` | List all saved command shortcuts |
| `!save <name>` | Save last commands as a named shortcut |
| `!forget <name>` | Remove a saved shortcut |
| `!<name>` | Run a saved shortcut instantly |
| `!exit` / `!q` | Quit yo-rust |
| `Y` / Enter | Confirm and run |
| `N` | Skip — rephrase and try again |
| `↑` / `↓` | Recall previous prompts in this session |
| `Ctrl+D` | Exit at any time |

### CLI flags

```bash
yo --dry          # Dry-run: show commands, never execute
yo -d             # Short form of --dry
yo --no-history   # Disable shell history appending this session
yo --no-context   # Disable multi-turn context this session
yo --help         # Show all flags
yo --version      # Show version
```

---

## 📁 Code structure

```
yo-rust/
├── src/
│   ├── main.rs        Entry point · REPL loop · execution · telemetry handles
│   ├── ai.rs          OpenRouter + Ollama HTTP · JSON envelope · intent detection
│   ├── config.rs      Load/save config · interactive setup wizard
│   ├── shell.rs       Shell detection matrix · cross-platform execution dispatch
│   ├── context.rs     Multi-turn conversation window (rolling N-turn buffer)
│   ├── history.rs     Shell history appending (zsh/bash/fish native formats)
│   ├── shortcuts.rs   Named command shortcuts (save, run, forget, persist)
│   ├── telemetry.rs   JSONBin.io data submission · thread handle management
│   ├── feedback.rs    !feedback / !fb subcommand handler and UI wizards
│   ├── cli.rs         clap argument parsing (--dry, --no-history, etc.)
│   └── ui.rs          Banner · help · suggestion display · feedback status
├── Cargo.toml         Rust manifest · annotated dependencies
├── yo.sh              Install (macOS/Linux/Git Bash)
├── update.sh          Update (macOS/Linux/Git Bash)
├── uninstall.sh       Uninstall (macOS/Linux/Git Bash)
├── install.ps1        Install (Windows PowerShell 5+/7+)
├── update.ps1         Update (Windows PowerShell)
├── uninstall.ps1      Uninstall (Windows PowerShell)
├── README.md          You're reading it
├── INSTALL.md         Full install / update / uninstall reference
├── CHANGELOG.md       Complete version history
└── LICENSE            MIT
```

---

## 🧠 How it works — architecture deep dive

### The core problem: getting reliable structured output from an LLM

The single hardest engineering challenge in this project is not the HTTP client, the Rust types, or the shell execution — it is making the AI produce **machine-parseable output, every single time, across every supported model**.

LLMs are text completers. They are trained to be helpful and conversational. Ask one to "give me a shell command to list files" and it will, with some probability, produce:

```
Here's how you can list files in your directory:

```bash
ls -la
```

This command will show all files including hidden ones, with permissions and timestamps.
```

None of that is directly executable as a shell command. The backticks, the explanation, the framing — all of it needs to be stripped before the command can run. And stripping it is fragile: what if the model uses different quote styles? What if it adds a numbered list? What if it includes a warning?

**The solution: a strict JSON envelope.**

Every request to the AI (both OpenRouter and Ollama) includes a system prompt that instructs the model to respond **only** with this exact JSON structure:

```json
{
  "commands": ["cmd1", "cmd2"],
  "explanation": "One plain-English sentence describing what these commands do."
}
```

This works reliably because:

1. **Modern LLMs are extensively trained on JSON.** They understand JSON structure better than any ad-hoc text format because they've seen billions of JSON examples during training.
2. **The schema separates machine-readable from human-readable.** The `commands` array contains exactly what gets executed. The `explanation` string contains exactly what gets displayed. There's no mixing.
3. **serde_json parses it deterministically.** If the JSON is valid and matches the schema, parsing succeeds. If not, we show the error — no ambiguity.
4. **Arrays handle multi-step answers naturally.** "First create the directory, then move the file" becomes `["mkdir -p /tmp/foo", "mv file.txt /tmp/foo/"]` — no need to split on newlines or semicolons.

We also strip accidental markdown fences (` ```json ... ``` `) before parsing, because some smaller models occasionally wrap output in them despite the system prompt saying not to. Belt and suspenders.

### Temperature 0.2 — why not 0?

Temperature controls how much randomness the model introduces when selecting each token. At temperature=1.0, the model samples broadly — great for creative writing, terrible for shell commands where `rm -rf /tmp` and `rm -rf /` are adjacent in the probability distribution.

We use **temperature 0.2** rather than 0 because:

- Temperature=0 is fully greedy (always picks the single most probable token), which can produce stuck, repetitive, or overly literal outputs for certain model architectures
- Temperature=0.2 is low enough that the model reliably produces the conventional, widely-understood form of a command rather than an exotic variant
- It's high enough to handle natural language variation in prompts without getting confused by unusual phrasing

Tested empirically across GPT-4o-mini, Claude 3 Haiku, Claude 3.5 Sonnet, and Llama 3.2 — 0.2 produces correct, safe commands in over 95% of cases.

### Context injection — the highest-leverage improvement

Without context, a model asked "open the downloads folder" has to guess the platform. It might produce `xdg-open ~/Downloads` (Linux), `open ~/Downloads` (macOS), or `explorer %USERPROFILE%\Downloads` (Windows). Two of those three will fail silently.

Every request to yo-rust is prefixed with:

```
System context: OS=macos ARCH=aarch64 CWD=/Users/paul/projects SHELL=zsh syntax=posix
```

With this, the model knows:
- Use `open` not `xdg-open` (macOS)
- Use `brew` not `apt` (macOS package manager)
- Use `pbcopy` / `pbpaste` for clipboard (macOS)
- Use paths relative to the actual CWD
- Use POSIX-compatible syntax (zsh)
- For Apple Silicon, prefer arm64 binary downloads

Four fields. Measurable improvement in correctness. The `syntax=posix` / `syntax=powershell` / `syntax=cmd` field is the most impactful: it explicitly tells the model which shell syntax family to use, eliminating the most common cross-platform errors.

### Multi-turn context — how follow-up prompts work

Without context, every prompt is stateless. "now do the same for /tmp" means nothing because the model doesn't know what "the same" refers to.

The `ConversationContext` module maintains a rolling window of the last N (default: 5) confirmed prompt/command pairs. Before each new AI request, these are injected into the `messages` array as prior `user`/`assistant` turns:

```json
[
  { "role": "system",    "content": "<system prompt>" },
  { "role": "user",      "content": "find log files older than 7 days" },
  { "role": "assistant", "content": "{\"commands\": [\"find . -name '*.log' -mtime +7\"]}" },
  { "role": "user",      "content": "now do the same for /tmp" }
]
```

The model sees the conversation history and resolves "same" correctly. This enables natural follow-up prompts:
- Pronouns: "delete them" → the files from the previous `find`
- Relative references: "now for staging" → swap the path
- Modifications: "without the -delete flag" → show but don't delete

The window is bounded (default 5 turns) to avoid unbounded token growth. Oldest turns are evicted first (FIFO) because the most recent context is most relevant to the current follow-up.

### Shell history integration — native formats matter

When yo-rust appends a confirmed command to your shell history, it writes it in the native format that shell expects — not a generic line of text. Getting the format wrong means the entry won't integrate cleanly with history search or timestamps.

- **zsh** uses EXTENDED_HISTORY format: `: <unix_timestamp>:0;<command>`. The `:0:` is the elapsed time (always 0 for externally appended entries). zsh's `fc` and history search read this format.
- **bash** uses plain `<command>\n` — one command per line, no metadata.
- **fish** uses a YAML-like format: `- cmd: <command>\n  when: <timestamp>`.

We detect which format to use from the `$SHELL` environment variable, `$ZDOTDIR` for non-standard zsh config locations, and `$HISTFILE` for custom bash history paths. Writing to the file does not update the live shell's in-memory buffer — press `history -r` (zsh) or `history -n` (bash) in a new window to pick up the entries, or simply open a new terminal.

### The telemetry thread-join problem

Background threads in Rust are attached to the main process. When the process exits, all threads are killed immediately — even threads that are in the middle of a network request.

The previous version of yo-rust used a fire-and-forget pattern:

```rust
std::thread::spawn(move || {
    submit_to_jsonbin(...);  // ~500ms network call
});
// Thread handle dropped — thread is detached
// User types !exit → process exits in ~10ms → thread killed
```

This is why the JSONBin collection was empty. Every telemetry entry was silently discarded before the HTTP POST could complete.

The fix: `submit_background()` returns a `JoinHandle`. The main REPL loop stores all handles in a `Vec<JoinHandle<()>>`. At every exit point (Ctrl-D, Ctrl-C, `!exit`), the code iterates over the handles and calls `.join()` on each one before returning. This gives in-flight HTTP requests time to complete.

```rust
// At every exit point:
for handle in pending_telemetry {
    let _ = handle.join();  // wait for HTTP POST to complete
}
return;
```

This pattern is important whenever you need "best-effort background work that should complete if the user exits cleanly." The thread is background (doesn't block the REPL) but its completion is guaranteed at exit.

### Command execution — why `sh -c` and not direct exec

When the AI suggests `find . -name "*.log" | xargs rm -f`, yo-rust needs to run that as-is. If we tried to exec it directly using `Command::new("find").args(...)`, we'd need to:

1. Parse the pipe character
2. Set up inter-process pipes between `find` and `xargs`
3. Handle shell globbing
4. Handle environment variable expansion
5. Handle `&&`, `||`, `;` command chaining

That's reimplementing a shell. Instead, we delegate to `sh -c` on Unix (or `powershell -Command` / `cmd /C` on Windows depending on detected shell), which handles all of these correctly in one line.

```rust
Command::new("sh")
    .arg("-c")
    .arg(cmd)         // the full command string as-is
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .status()
```

`Stdio::inherit()` is equally important. Without it, interactive programs (`vim`, `htop`, `less`, `fzf`) can't attach to a TTY and will either crash or produce no output. Streaming programs (`cargo build`, `npm install`) would buffer all output until completion instead of showing it in real time. And colour-aware programs (`ls`, `grep --color`) would detect a non-TTY stdout and disable colour.

### Windows PowerShell — the `cargo` stderr problem

This is the most counterintuitive bug we fixed (v2.2.0). The original `install.ps1` had:

```powershell
$ErrorActionPreference = "Stop"
& cargo build --release 2>&1 | Out-Null
```

The intent was: suppress noisy output and stop on error. The result: the script always crashed during the build.

The root cause is a PowerShell 5.1 behaviour: when stderr output from a native executable is redirected with `2>&1`, each line is captured as an `ErrorRecord` object. With `$ErrorActionPreference = "Stop"`, **any** `ErrorRecord` immediately terminates the script — even non-errors.

`cargo` writes all of its progress output (`Updating crates.io index`, `Compiling foo v1.0`, etc.) to **stderr**, even on a completely successful build. So the very first progress message triggered a `TerminatingError` and killed the script.

The fix: remove `$ErrorActionPreference = "Stop"` entirely. Remove `2>&1`. Let cargo's output flow directly to the terminal. Check `$LASTEXITCODE` after `cargo` finishes to detect real failures. This is the correct idiom for native commands in PowerShell.

---

## 🔬 Lessons learned — complete list

Building yo-rust from scratch and iterating through a dozen real user sessions and bug reports produced a set of hard-won lessons. These are real observations, not platitudes.

### On LLM prompt engineering

**Structured output is not just convenient — it's load-bearing.** The JSON envelope is not a nicety. It is the foundation on which everything else works. Without it, the entire command extraction pipeline is unreliable. Invest in the system prompt first; optimise everything else second.

**State your output format twice.** LLMs tend to "forget" instructions that appeared many tokens ago. If you state the JSON schema once at the top of the system prompt, models occasionally violate it on longer completions. Stating it twice (once in the rules, once as a concrete schema example) dramatically improves compliance, especially with smaller models.

**Temperature 0.2 is better than temperature 0 for tool use.** Full greedy decoding (temperature 0) produces too-literal outputs and can get stuck in degenerate loops on some model architectures. A small amount of randomness at 0.2 allows the model to handle paraphrased or unusual prompts without breaking the output format.

**Context window position matters.** The most recent user message gets the most attention from the model. System prompt instructions can be "attenuated" by long conversation histories. For important constraints (like the JSON schema), restating them in the user message or close to the end of the prompt helps on weaker models.

**Inject platform context, not tool inventory.** We don't tell the model "you have access to brew, apt, dnf, etc." — we tell it "OS=macos ARCH=aarch64 SHELL=zsh syntax=posix". From that, a good model infers the available tools correctly. Context about the environment is more useful than an explicit tool list.

### On Rust for CLI tools

**`cargo check` is your best friend during development.** It type-checks the entire project without producing a binary, completing in a fraction of the time of `cargo build`. Use it constantly while writing code. Full `cargo build --release` only when you need the binary.

**Avoid `2>&1` on native commands in PowerShell scripts.** This was the most damaging lesson. In PowerShell 5.1, redirecting stderr with `2>&1` converts stderr lines to ErrorRecord objects. Combined with `$ErrorActionPreference = "Stop"`, this kills the script on any stderr output, including normal progress messages. Always let native commands write to their natural outputs and check `$LASTEXITCODE` instead.

**Thread handles must be joined before process exit.** Detached threads (dropped `JoinHandle`) are killed when the process exits. For any background work that must complete (network requests, file flushes), store the handle and join it at every exit point. This is a subtle correctness requirement that is easy to miss.

**`#[serde(default)]` is the key to forward-compatible config files.** Every new field added to a config struct should have `#[serde(default)]`. This means that existing config files (without the new field) will deserialise successfully, with the field taking its default value. Without this, updating the app breaks every existing user's config.

**Error types: use `Box<dyn Error>` early, refine later.** `Box<dyn std::error::Error>` as a return type lets you use `?` on any error type without defining a custom error enum. For a project of this size, it's the right starting point. Custom error types add value when you need to match on error variants — which yo-rust doesn't yet need.

**`Stdio::inherit()` for interactive child processes is non-negotiable.** Any tool that runs other CLI programs needs to inherit stdio, not capture it. Capturing breaks TUI programs, buffering breaks streaming output, and non-TTY detection breaks colour output in dozens of tools. Learn this once, apply it everywhere.

### On Windows support

**`curl` is not curl on Windows.** PowerShell has a built-in alias `curl → Invoke-WebRequest` that looks like curl but isn't. It doesn't accept `-fsSL` flags. Every Unix install instruction that starts with `curl -fsSL ... | bash` fails silently (or loudly) on Windows PowerShell. The correct Windows install idiom is `iwr -useb <url> | iex`. This is now prominently documented and there's a native `install.ps1` for Windows users.

**PowerShell 5 and 7 are meaningfully different.** PS5 is "Windows PowerShell" (built into Windows, available everywhere). PS7 is "PowerShell" (cross-platform, opt-in install). The key difference for yo-rust: PS7 supports `&&` for command chaining, PS5 does not. Users of PS5 who get a command with `&&` will see a cryptic error. Detecting the version and adjusting the syntax hint eliminates this entirely.

**Git Bash is POSIX on Windows, but it's still Windows.** A user running Git Bash has `$SHELL=/usr/bin/bash`, has access to standard Unix tools via MSYS2, and expects POSIX-compatible commands. But the underlying filesystem uses Windows paths and the process tree is Windows. We detect Git Bash via `$MSYSTEM` and treat it as POSIX — giving the user `find`, `grep`, `awk` etc. rather than PowerShell cmdlets.

### On the telemetry pipeline

**Fire-and-forget is wrong for anything that must complete.** The original telemetry implementation spawned a thread and dropped the handle. Threads are not daemons — they don't run after the process exits. Every telemetry entry was silently lost. The fix (storing JoinHandles and joining at exit) is the correct pattern for background work that must complete.

**Silent error swallowing makes debugging impossible.** Every error in the original telemetry was swallowed with `Err(_) => {}`. When nothing appeared in the JSONBin dashboard, there was no way to know if the entries were being sent and rejected, or never sent at all. The `YODEBUG=1` environment variable now prints full HTTP payloads and response codes to stderr. Observability is not optional.

**Write-only API keys are the correct security model for telemetry.** A key that can only create new bins cannot read, update, or delete anything — even its own creations. This is safe to ship in a compiled binary. The worst-case scenario if it were misused: someone adds junk entries to the collection. That's noise Paul can filter, not a breach.

**Separate bins per entry, not one appended bin.** JSONBin's model is one document per bin. We create one bin per telemetry entry rather than trying to append to a single document. This is actually better: entries are independent, can't be correlated by bin ID, and the collection view in the dashboard shows them cleanly as individual records with timestamps.

---

## 📊 Community data & telemetry

yo-rust can optionally share anonymised data about which prompts produced useful commands. This builds a real-world dataset reviewed weekly to improve the AI system prompt.

**What gets shared** (only when you opt in, default is OFF):

| Field | Example |
|---|---|
| Your prompt | `"find log files older than 7 days"` |
| Commands that ran | `["find . -name '*.log' -mtime +7"]` |
| AI model used | `"openai/gpt-4o-mini"` |
| OS + shell | `"macos"` + `"zsh"` |
| Whether it worked | `true` |
| yo-rust version | `"v2.3.3"` |
| Timestamp | `"2026-03-22T21:00:00Z"` |

**What is never shared:** API keys, file paths, working directory, command output, username, hostname, or any identity.

Data goes to a private [JSONBin.io](https://jsonbin.io) collection via a write-only Access Key — it can create bins but cannot read, update, or delete. To enable: `!feedback on`. To see the full status: `!feedback`. To run a live connectivity test: `!feedback test`.

You can also configure your own personal JSONBin collection to keep a private copy of your own command history: `!feedback personal`.

Debug mode for troubleshooting: `YODEBUG=1 yo`.

---

## 🔑 OpenRouter model recommendations

| Model | Cost | Best for |
|---|---|---|
| `openai/gpt-4o-mini` | ~$0.15/1M tokens | ★ Default — fast, reliable, follows JSON schema |
| `openai/gpt-4o` | ~$2.50/1M tokens | Complex multi-step requests |
| `anthropic/claude-3.5-sonnet` | ~$3/1M tokens | Tricky, context-heavy tasks |
| `anthropic/claude-3-haiku` | ~$0.25/1M tokens | Speed-critical workflows |
| `meta-llama/llama-3.3-70b-instruct:free` | Free | Getting started, simple tasks |

Get a key: **[openrouter.ai/keys](https://openrouter.ai/keys)**

---

## 🌟 v2 — What changed in the major version

| Feature | Description |
|---|---|
| 🏠 **Ollama backend** | Local model inference — no API key, no network, complete privacy |
| 🔁 **Multi-turn context** | Follow-up prompts resolve correctly — "now do the same for X" works |
| 📜 **Shell history** | Confirmed commands appended to `~/.zsh_history` / `~/.bash_history` |
| 🧪 **Dry-run mode** | `yo --dry` — see every suggested command before any execution |
| 🪝 **Post-execution feedback** | "Did that work?" loop — AI refines on failure |
| 🪟 **Windows support** | PS5, PS7, cmd.exe, Git Bash — detected, correct syntax generated |
| 🐚 **Precise shell detection** | `shell.rs` module — full detection matrix, `syntax=` context hint |
| 💾 **Named shortcuts** | `!save`, `!forget`, instant replay with `!<name>` |
| 📊 **Community telemetry** | Opt-in JSONBin.io data sharing to improve the tool over time |

---

## 📝 Changelog

> Full history: **[CHANGELOG.md](CHANGELOG.md)**

### 🔖 v2.3.3 — 2026-03-22
- 🐛 Fixed telemetry thread race — JoinHandles now stored and joined at exit
- 🔍 Added `YODEBUG=1` environment variable for HTTP-level telemetry diagnostics
- ✨ `!feedback test` — send a live test entry and verify it arrives immediately
- 📝 README: expanded Rust rationale, full lessons learned, architecture deep dives

### 🔖 v2.3.1 — 2026-03-22
- ✨ `!feedback` / `!fb` shortcut with full subcommand UI
- 🔗 JSONBin.io collection `yo-rust-telemetry` live and accepting entries

### 🔖 v2.3.0 — 2026-03-22
- ✨ Community telemetry via JSONBin.io (opt-in)
- ✨ Personal JSONBin support for private command history

### 🔖 v2.2.0 — 2026-03-22
- 🐛 Fixed Windows PS5.1 `TerminatingError` — `cargo build` killed by `$ErrorActionPreference`
- ✨ Named command shortcuts (`!save`, `!forget`, `!<name>`)

### 🔖 v2.1.0 — 2026-03-22
- ✨ Native PowerShell installer (`install.ps1`, `update.ps1`, `uninstall.ps1`)

---

## 🤝 Contributing

```bash
git checkout -b feat/your-feature
git commit -m 'feat: describe your change'
git push origin feat/your-feature
# → open a Pull Request
```

Ideas still on the list:
- `--stop-on-error` flag for multi-command sequences
- Keychain/credential manager storage for the API key
- PowerShell completion script for `!` shortcut names
- Configurable temperature per session

---

## 📜 License

MIT — see [LICENSE](LICENSE).

---

## 👤 Author

Made with ❤️ by **Paul Fleury** — designed and built in collaboration with **[Perplexity Computer](https://www.perplexity.ai)**.

- 🌐 **[paulfleury.com](https://paulfleury.com)**
- 🔗 **[linkedin.com/in/paulfxyz](https://www.linkedin.com/in/paulfxyz/)**
- 🐙 **[@paulfxyz](https://github.com/paulfxyz)**
- 📧 **[hello@paulfleury.com](mailto:hello@paulfleury.com)**

---

<div align="center">

⭐ **If yo-rust saved you time, drop a star — it helps others find it.** ⭐

</div>
