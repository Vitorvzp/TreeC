use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io::Stdout, path::{Path, PathBuf}, time::Duration};

use super::{screens, wizard};

/// All available TUI screens
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Dashboard,
    Agents,
    Tasks,
    BrainViewer,
    SharedMemory,
    Changelog,
    CreateAgent, // Wizard mode
}

impl Screen {
    pub fn label(&self) -> &'static str {
        match self {
            Screen::Dashboard => "Dashboard",
            Screen::Agents => "Agents",
            Screen::Tasks => "Tasks",
            Screen::BrainViewer => "Brain Viewer",
            Screen::SharedMemory => "Shared Memory",
            Screen::Changelog => "Changelog",
            Screen::CreateAgent => "Create Agent",
        }
    }

    pub fn all_nav() -> Vec<Screen> {
        vec![
            Screen::Dashboard,
            Screen::Agents,
            Screen::Tasks,
            Screen::BrainViewer,
            Screen::SharedMemory,
            Screen::Changelog,
        ]
    }
}

/// Global application state
pub struct App {
    pub root: PathBuf,
    pub current_screen: Screen,
    pub nav_index: usize,
    pub scroll: u16,
    pub should_quit: bool,
    pub wizard_state: wizard::WizardState,
    pub status_msg: String,
    /// Cached file content for brain viewer
    pub viewer_content: String,
    pub viewer_title: String,
}

impl App {
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            current_screen: Screen::Dashboard,
            nav_index: 0,
            scroll: 0,
            should_quit: false,
            wizard_state: wizard::WizardState::new(),
            status_msg: String::new(),
            viewer_content: String::new(),
            viewer_title: String::new(),
        }
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        self.current_screen = screen;
        self.scroll = 0;
    }

    pub fn nav_next(&mut self) {
        let nav = Screen::all_nav();
        self.nav_index = (self.nav_index + 1) % nav.len();
        self.navigate_to(nav[self.nav_index].clone());
    }

    pub fn nav_prev(&mut self) {
        let nav = Screen::all_nav();
        let len = nav.len();
        self.nav_index = (self.nav_index + len - 1) % len;
        self.navigate_to(nav[self.nav_index].clone());
    }

    pub fn scroll_down(&mut self) { self.scroll = self.scroll.saturating_add(3); }
    pub fn scroll_up(&mut self) { self.scroll = self.scroll.saturating_sub(3); }

    #[allow(dead_code)]
    pub fn read_brain_file(&mut self, relative_path: &str) {
        let path = self.root.join(".brain").join(relative_path);
        self.viewer_title = relative_path.to_string();
        self.viewer_content = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| format!("(file not found: .brain/{})", relative_path));
        self.navigate_to(Screen::BrainViewer);
    }
}

/// Main TUI loop
pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    root: &Path,
) -> Result<(), String> {
    let mut app = App::new(root);

    loop {
        terminal.draw(|frame| screens::render(frame, &app))
            .map_err(|e| format!("Draw error: {}", e))?;

        if event::poll(Duration::from_millis(200))
            .map_err(|e| format!("Event poll error: {}", e))?
        {
            if let Event::Key(key) = event::read()
                .map_err(|e| format!("Event read error: {}", e))?
            {
                handle_key(&mut app, key.code, key.modifiers);
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyCode, modifiers: KeyModifiers) {
    // Wizard mode has its own key handling
    if app.current_screen == Screen::CreateAgent {
        wizard::handle_wizard_key(app, key, modifiers);
        return;
    }

    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Tab => app.nav_next(),
        KeyCode::BackTab => app.nav_prev(),
        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
        KeyCode::Char('n') if app.current_screen == Screen::Agents => {
            app.navigate_to(Screen::CreateAgent);
            app.wizard_state = wizard::WizardState::new();
        }
        KeyCode::Char('1') => { app.nav_index = 0; app.navigate_to(Screen::Dashboard); }
        KeyCode::Char('2') => { app.nav_index = 1; app.navigate_to(Screen::Agents); }
        KeyCode::Char('3') => { app.nav_index = 2; app.navigate_to(Screen::Tasks); }
        KeyCode::Char('4') => { app.nav_index = 3; app.navigate_to(Screen::BrainViewer); }
        KeyCode::Char('5') => { app.nav_index = 4; app.navigate_to(Screen::SharedMemory); }
        KeyCode::Char('6') => { app.nav_index = 5; app.navigate_to(Screen::Changelog); }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_app_navigation() {
        let dir = tempdir().unwrap();
        let mut app = App::new(dir.path());
        assert_eq!(app.current_screen, Screen::Dashboard);
        app.nav_next();
        assert_eq!(app.current_screen, Screen::Agents);
        app.nav_prev();
        assert_eq!(app.current_screen, Screen::Dashboard);
    }

    #[test]
    fn test_app_scroll() {
        let dir = tempdir().unwrap();
        let mut app = App::new(dir.path());
        app.scroll_down();
        assert_eq!(app.scroll, 3);
        app.scroll_up();
        assert_eq!(app.scroll, 0);
    }

    #[test]
    fn test_screen_labels() {
        assert_eq!(Screen::Dashboard.label(), "Dashboard");
        assert_eq!(Screen::Agents.label(), "Agents");
    }
}
