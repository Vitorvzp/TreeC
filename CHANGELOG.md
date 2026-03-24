# Changelog

All notable changes to TreeC-Rust will be documented in this file.

## [Unreleased]

## [0.5.0] - 2026-03-24

### Fixed

- **CI quality** — Applied `cargo fmt` across all source files. Fixed 5 clippy warnings promoted
  to errors by `-D warnings`: `dead_code` on `ScanEntry::is_dir`, `if_same_then_else` in
  `brain.rs`, `needless_borrows_for_generic_args` in `generator.rs`, `collapsible_if` and
  `too_many_arguments` in `main.rs`. CI pipeline now passes cleanly on all 3 platforms.

### Added

- **Ollama provider** (`src/neural.rs`) — Local AI deployment via Ollama. Calls
  `http://localhost:11434/api/chat` with JSON format. No API key required.
  Configure with: `treec --config-neural ollama llama3.2`
  Works with any Ollama model that supports JSON output (llama3.2, mistral, qwen2.5, etc.)
- **`--brain-files` flag** (`src/main.rs`) — Selective brain regeneration. Specify which files
  to regenerate as a comma-separated list of keys.
  Example: `treec --neural-link --brain-files context,architecture,tasks`
  Builds a focused AI prompt requesting only the specified sections.
- **Cost estimation** (`src/main.rs`) — `estimate_cost_usd()` calculates and displays estimated
  USD cost based on token count × model price (per 1M tokens). Shown alongside token/context
  window info before every API call.
- **`.github/workflows/ci.yml`** — CI pipeline for PRs and pushes. Runs `cargo fmt --check`,
  `cargo clippy -D warnings`, and `cargo build --release` on Ubuntu, Windows, and macOS.
  Includes a changelog version check step.

### Changed

- **`src/scanner.rs`** — Complete rewrite using the `ignore` crate (v0.4). Replaces `walkdir` +
  manual Regex gitignore parser. Now supports full `.gitignore` semantics including negation (`!`
  patterns), anchoring, and `.git/info/exclude`. `IncludeHiddenDirs` maps directly to
  `WalkBuilder::hidden()`. Code is ~40% shorter.
- **`src/main.rs`** — File analysis loop in `run_scan_pipeline` is now parallelized with `rayon`.
  Uses `par_iter()` on scan entries; `indicatif::ProgressBar` is thread-safe (Arc-backed).
  `--config-neural` now accepts 1 or 2 arguments (`num_args = 1..=2`) to support
  Ollama without requiring a dummy API key.
- **`Cargo.toml`** — Bumped to v0.5.0. Removed `regex` and `walkdir` dependencies.
  Added `ignore = "0.4"` and `rayon = "1.10"`.

## [0.4.0] - 2026-03-24

### Added

- **`treec --status`** — New diagnostic command. Displays: TreeC.toml presence, `.brain/` last update,
  Tree.md presence, configured provider/model, API key source (env var vs file), and `.gitignore`
  security check.
- **Token estimation** — Before any AI API call, estimates token count (`len / 3.5`) and compares
  against the model's context window. Exits with error if >90%, warns if >70%.
  Context windows mapped: Gemini 2.0 (1M), GPT-4.1 (128k), Claude Sonnet 4 (200k).
- **Auto `.gitignore` protection** — Running `treec --config-neural` now automatically adds
  `TreeC.toml` to `.gitignore` if it isn't already protected, preventing accidental API key commits.
- **Progress bar** (`indicatif` 0.17) — Spinner + progress bar during file analysis in the scan
  pipeline. Auto-clears on completion.
- **`.github/workflows/release.yml`** — Automated GitHub Actions pipeline. On semver tag push,
  builds binaries for Windows x64, Linux x64 (musl static), macOS ARM, and macOS Intel, then
  creates a GitHub Release with extracted CHANGELOG notes.
- **`roadmap.md`** — Full project roadmap with 35+ improvements organized by category,
  priority, and target version. Covers ✅ done, 🟡 pending, and 💡 future ideas.

### Changed

- **`Cargo.toml`** — Bumped to v0.4.0. Added `license`, `keywords`, `categories`, `readme` fields
  for crates.io readiness. Added `indicatif = "0.17"` dependency.
- **`src/main.rs`** — `--config-neural` now shows truncated key hint for env var usage.
  `--config` template updated with `TREEC_API_KEY` tip comment.

## [0.3.0] - 2026-03-24

### Security

- **`src/config.rs`** — Added `TREEC_API_KEY` environment variable support.
  API key resolution order: `TREEC_API_KEY` env var (priority) → `ApiKey` in `[NeuralLink]` section.
  Prevents accidental credential exposure when committing `TreeC.toml` to version control.

### Changed

- **`src/config.rs`** — Replaced manual Regex-based TOML parsing with the `toml` crate (v0.8) + serde.
  Eliminates fragile section-unaware regex matching. Now correctly respects TOML structure and section
  boundaries. Adds `IncludeHiddenDirs` field support.
- **`src/analyzer.rs`** — Improved binary file detection heuristic.
  Now uses two criteria: (a) presence of any null byte (0x00), AND (b) >30% non-printable bytes.
  Reduces false positives on UTF-16 encoded text files and false negatives on binary files
  that begin with ASCII-compatible bytes.
- **`src/scanner.rs`** — `IncludeHiddenDirs` config option now controls hidden directory scanning.
  When `true`, directories starting with `.` (e.g. `.github`, `.vscode`) are included in the scan.
- **`TreeC.toml`** — Added `IncludeHiddenDirs = false` to `[General]` section.
- **`src/main.rs`** — Default config template updated to include `IncludeHiddenDirs`.
- **`Cargo.toml`** — Added `toml = "0.8"` dependency.

### Added

- **`context.md`** — Project second brain with Obsidian wikilinks: overview, architecture, commands,
  identified issues and their fix status.
- **`memory.md`** — Long-term project memory log (append-only).

## [0.2.0] - 2026-03-24

### Added

- **`src/brain.rs`** — .brain/ directory structure manager
  - Creates 17 structured files (index, context, architecture, memory, changelog, roadmap, decisions, prompt, tree, readme, tasks + knowledge/*)
  - Seed content with Obsidian-compatible [[wikilinks]]
  - Hardcoded agent prompt for AI behavior rules
  - Append-only memory system
- **`src/neural.rs`** — AI API integration (Gemini)
  - Sends Tree.md content + system prompt to Gemini API
  - Expects structured JSON response with 12 brain file contents
  - Parses response and populates .brain/ files
  - Uses `ureq` for HTTP requests with JSON body
- **`treec --neural-link`** — Create AI Second Brain (.brain/)
- **`treec --update-brain`** — Incremental brain update (tree, memory, changelog + optional AI refresh)
- **`treec --config-neural <API_KEY>`** — Save API key to TreeC.toml
- **`treec --config`** — Create default TreeC.toml configuration
- **`treec --obsidian`** — Setup Obsidian vault inside .brain/ (app, graph, workspace configs)
- **`treec --clean`** — Remove all generated files (Tree.md, Structure.*, .brain/)

### Changed

- **`src/config.rs`** — Added `[NeuralLink]` section parsing (ApiKey, Model, Provider) and `save_api_key()` function
- **`src/main.rs`** — Full CLI rewrite with all 8 commands, modular handler functions
- **`Cargo.toml`** — Added `ureq` dependency for HTTP requests

## [0.1.0] - 2026-03-23

### Added

- **`src/config.rs`** — Regex-based TOML parser for `TreeC.toml`
  - Parses `[General]`, `[Exports]`, `[Ignore]` sections
  - Supports integer, boolean, and string array values
  - Falls back to sensible defaults when file is missing
- **`src/scanner.rs`** — Iterative directory walker using `walkdir`
  - Respects config-defined ignore rules (folders, extensions, files)
  - Hardcoded automatic exclusions (Tree.md, Structure.json, Structure.txt, TreeC.toml, treec, .git, .gitignore)
  - `.gitignore` pattern parsing with regex-based glob-to-regex conversion
  - Sorts output: directories first, then files, all alphabetical
- **`src/analyzer.rs`** — File analysis pipeline
  - Null Byte Detection: reads first 1024 bytes, flags binary if `0x00` found
  - Fast LOC counting: scans for `0x0A` bytes in 8KB chunks (no UTF-8 overhead)
  - Language detection: 40+ extension-to-language mappings (Rust, Python, JS, TS, Go, etc.)
  - Graceful error handling: skips unreadable/locked files with stderr warning
- **`src/generator.rs`** — Output generation engine
  - `Tree.md`: Full structured Markdown with `# Root`, `## Summary`, `## Tree`, `## Files` sections
  - `Structure.json`: Pretty-printed JSON with project name, stats, and file metadata
  - `Structure.txt`: ASCII tree only
  - ASCII tree builder using BTreeMap with `├──`, `└──`, `│` characters
- **`src/main.rs`** — CLI router using `clap` derive
  - Full execution pipeline: config → scan → analyze → generate → write → report
  - Accepts optional root path argument (defaults to `.`)
  - Performance timing and summary output
- **`TreeC.toml`** — Default configuration template with all sections

### Changed

- **`Cargo.toml`** — Fixed `edition` from `"2026"` (invalid) to `"2021"`, added `[[bin]]` section to rename binary to `treec`, added `description` and `repository` fields
- **`.gitignore`** — Rewritten for publishable Rust project (build artifacts, generated outputs, IDE/OS files, local context notes)
- **`README.md`** — Bilingual README (English + Portuguese) for GitHub with badges, install/usage instructions, config, and feature highlights
