# Changelog

## [2026-03-24 HH:MM] — Fix CI: fmt + clippy errors

**What:** Corrigidos 2 motivos de falha no CI — formatação e 5 erros de clippy.

**Why:** CI falhava em `cargo fmt --check` (arquivos mal formatados) e `cargo clippy -- -D warnings` (5 warnings tratados como erros).

**Files:**
- `src/scanner.rs` — `#[allow(dead_code)]` em `ScanEntry::is_dir`
- `src/brain.rs` — removido if/else com blocos idênticos
- `src/generator.rs` — removido borrow desnecessário em `root.join(...)`
- `src/main.rs` — colapsado if aninhado; `#[allow(clippy::too_many_arguments)]` em `handle_update_brain`
- Todos os `.rs` — `cargo fmt` aplicado
