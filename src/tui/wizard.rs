use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::Frame;
use tui_textarea::TextArea;

use super::app::App;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum WizardStep {
    Name,
    Role,
    Specialties,
    Prompt,
    Confirm,
}

#[allow(dead_code)]
pub struct WizardState {
    pub step: WizardStep,
    pub name: String,
    pub role_index: usize,
    pub custom_role: String,
    pub custom_role_active: bool,
    pub specialties: String,
    pub prompt_textarea: TextArea<'static>,
    pub error: String,
}

impl WizardState {
    pub fn new() -> Self {
        Self {
            step: WizardStep::Name,
            name: String::new(),
            role_index: 0,
            custom_role: String::new(),
            custom_role_active: false,
            specialties: String::new(),
            prompt_textarea: TextArea::default(),
            error: String::new(),
        }
    }
}

pub fn handle_wizard_key(_app: &mut App, _key: KeyCode, _modifiers: KeyModifiers) {
    // Implemented in Chunk 4
}

pub fn render_wizard(frame: &mut Frame, _app: &App) {
    use ratatui::widgets::{Block, Borders, Paragraph};
    let area = frame.area();
    let placeholder = Paragraph::new("Agent creation wizard — coming in Chunk 4")
        .block(Block::default().title(" Create Agent ").borders(Borders::ALL));
    frame.render_widget(placeholder, area);
}
