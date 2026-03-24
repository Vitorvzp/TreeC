# TreeC — Project Overview

## Description
CLI tool (Rust) for scanning repositories and generating structured documentation. Optionally sends the result to an AI API (Gemini, OpenAI, Claude, Ollama) to populate a `.brain/` knowledge base.

## Tech Stack
- **Language**: Rust (edition 2021)
- **Version**: 0.5.0
- **Key dependencies**: clap (CLI), ignore (gitignore-aware walker), rayon (parallel analysis), serde/serde_json, toml, ureq (HTTP), indicatif (progress bars), chrono

## Architecture
```
main.rs         — CLI parsing, orchestration, command handlers
scanner.rs      — Directory walking (ignore crate, gitignore support)
analyzer.rs     — Binary detection, LOC counting, language detection
generator.rs    — Markdown/JSON/TXT artifact generation
config.rs       — TreeC.toml loading/saving (serde)
neural.rs       — AI API calls (Gemini, OpenAI, Claude, Ollama) → .brain/ files
brain.rs        — .brain/ directory initialization and file management
```

## Key Commands
- `treec` — scan and generate Tree.md
- `treec --neural-link` — scan + AI generates .brain/
- `treec --update-brain` — update existing .brain/
- `treec --config-neural <PROVIDER> <KEY>` — configure AI
- `treec --obsidian` — setup Obsidian vault for .brain/
- `treec --status` — show config/brain/key status
- `treec --clean` — remove generated artifacts

## CI
GitHub Actions: `build-and-test` (ubuntu/windows/macos) + `changelog-check`.
Steps: `cargo fmt --check` → `cargo clippy -- -D warnings` → `cargo build` → `cargo build --release`.
