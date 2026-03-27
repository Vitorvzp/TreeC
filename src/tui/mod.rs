use std::path::Path;

pub mod app;
pub mod screens;
pub mod wizard;

pub fn run_tui(_root: &Path) -> Result<(), String> {
    Err("TUI not yet implemented — coming in v1.0.0".to_string())
}
