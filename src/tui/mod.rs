pub mod app;
pub mod screens;
pub mod wizard;

use std::path::Path;

/// Entry point: run the TUI application.
pub fn run_tui(root: &Path) -> Result<(), String> {
    let mut terminal = setup_terminal()?;
    let result = app::run_app(&mut terminal, root);
    restore_terminal(&mut terminal)?;
    result
}

fn setup_terminal() -> Result<ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>, String> {
    use crossterm::{execute, terminal::*};
    crossterm::terminal::enable_raw_mode()
        .map_err(|e| format!("Failed to enable raw mode: {}", e))?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)
        .map_err(|e| format!("Failed to enter alternate screen: {}", e))?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    ratatui::Terminal::new(backend)
        .map_err(|e| format!("Failed to create terminal: {}", e))
}

fn restore_terminal(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> Result<(), String> {
    use crossterm::{execute, terminal::*};
    crossterm::terminal::disable_raw_mode()
        .map_err(|e| format!("Failed to disable raw mode: {}", e))?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .map_err(|e| format!("Failed to leave alternate screen: {}", e))?;
    terminal.show_cursor()
        .map_err(|e| format!("Failed to show cursor: {}", e))
}
