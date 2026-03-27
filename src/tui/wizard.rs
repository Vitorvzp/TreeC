use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};
use tui_textarea::TextArea;

use crate::agent::AgentMeta;
use super::app::App;

/// Wizard step
#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep {
    Name,
    Role,
    Specialties,
    Prompt,
    Confirm,
}

/// Predefined roles for selection
const ROLES: &[&str] = &[
    "Backend Developer",
    "Frontend Developer",
    "Architect",
    "QA / Test Engineer",
    "Documentation Writer",
    "DevOps Engineer",
    "Security Engineer",
    "Database Engineer",
    "Custom...",
];

pub struct WizardState {
    pub step: WizardStep,
    pub name: String,
    #[allow(dead_code)]
    pub name_cursor: usize,
    pub role_index: usize,
    pub custom_role: String,
    pub custom_role_active: bool,
    pub specialties: String,
    #[allow(dead_code)]
    pub specialties_cursor: usize,
    pub prompt_textarea: TextArea<'static>,
    pub error: String,
}

impl WizardState {
    pub fn new() -> Self {
        let mut ta = TextArea::default();
        ta.set_placeholder_text("Describe the agent's behavior and focus...");
        Self {
            step: WizardStep::Name,
            name: String::new(),
            name_cursor: 0,
            role_index: 0,
            custom_role: String::new(),
            custom_role_active: false,
            specialties: String::new(),
            specialties_cursor: 0,
            prompt_textarea: ta,
            error: String::new(),
        }
    }

    pub fn resolved_role(&self) -> String {
        if self.role_index == ROLES.len() - 1 {
            self.custom_role.clone()
        } else {
            ROLES[self.role_index].to_string()
        }
    }

    pub fn resolved_specialties(&self) -> Vec<String> {
        self.specialties
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn resolved_prompt(&self) -> String {
        self.prompt_textarea.lines().join("\n")
    }

    pub fn validate(&self) -> Option<String> {
        if self.name.trim().is_empty() {
            return Some("Agent name cannot be empty.".to_string());
        }
        if self.name.contains(' ') {
            return Some("Agent name cannot contain spaces. Use hyphens: my-agent".to_string());
        }
        if self.resolved_role().trim().is_empty() {
            return Some("Role cannot be empty.".to_string());
        }
        None
    }
}

/// Handle keyboard input in wizard mode
pub fn handle_wizard_key(app: &mut App, key: KeyCode, modifiers: KeyModifiers) {
    // Clone the step to avoid borrow conflicts when we need to mutate app fields
    let step = app.wizard_state.step.clone();

    match step {
        WizardStep::Name => match key {
            KeyCode::Esc => {
                app.current_screen = super::app::Screen::Agents;
            }
            KeyCode::Enter => {
                if app.wizard_state.name.trim().is_empty() {
                    app.wizard_state.error = "Name cannot be empty.".to_string();
                } else if app.wizard_state.name.contains(' ') {
                    app.wizard_state.error = "Use hyphens instead of spaces.".to_string();
                } else {
                    app.wizard_state.error.clear();
                    app.wizard_state.step = WizardStep::Role;
                }
            }
            KeyCode::Backspace => {
                app.wizard_state.name.pop();
            }
            KeyCode::Char(c) => {
                app.wizard_state.name.push(c);
            }
            _ => {}
        },

        WizardStep::Role => match key {
            KeyCode::Esc => {
                app.wizard_state.step = WizardStep::Name;
            }
            KeyCode::Up => {
                app.wizard_state.role_index = app.wizard_state.role_index.saturating_sub(1);
            }
            KeyCode::Down => {
                app.wizard_state.role_index =
                    (app.wizard_state.role_index + 1).min(ROLES.len() - 1);
            }
            KeyCode::Enter => {
                let is_custom = app.wizard_state.role_index == ROLES.len() - 1;
                app.wizard_state.custom_role_active = is_custom;
                if !is_custom {
                    app.wizard_state.step = WizardStep::Specialties;
                }
            }
            KeyCode::Backspace if app.wizard_state.custom_role_active => {
                app.wizard_state.custom_role.pop();
            }
            KeyCode::Char(c) if app.wizard_state.custom_role_active => {
                app.wizard_state.custom_role.push(c);
            }
            _ => {}
        },

        WizardStep::Specialties => match key {
            KeyCode::Esc => {
                app.wizard_state.step = WizardStep::Role;
            }
            KeyCode::Enter => {
                app.wizard_state.step = WizardStep::Prompt;
            }
            KeyCode::Backspace => {
                app.wizard_state.specialties.pop();
            }
            KeyCode::Char(c) => {
                app.wizard_state.specialties.push(c);
            }
            _ => {}
        },

        WizardStep::Prompt => {
            match key {
                KeyCode::Esc => {
                    app.wizard_state.step = WizardStep::Specialties;
                }
                KeyCode::Enter if modifiers.contains(KeyModifiers::CONTROL) => {
                    app.wizard_state.step = WizardStep::Confirm;
                }
                _ => {
                    use crossterm::event::{KeyEvent, KeyEventKind, KeyEventState};
                    let ke = KeyEvent {
                        code: key,
                        modifiers,
                        kind: KeyEventKind::Press,
                        state: KeyEventState::NONE,
                    };
                    app.wizard_state.prompt_textarea.input(ke);
                }
            }
        }

        WizardStep::Confirm => match key {
            KeyCode::Esc => {
                app.wizard_state.step = WizardStep::Prompt;
            }
            KeyCode::Enter | KeyCode::Char('y') => {
                save_wizard_to_brain(app);
            }
            KeyCode::Char('n') => {
                app.current_screen = super::app::Screen::Agents;
            }
            _ => {}
        },
    }
}

/// Save wizard result to _pending/ using brain functions
fn save_wizard_to_brain(app: &mut App) {
    // Validate first
    if let Some(err) = app.wizard_state.validate() {
        app.wizard_state.error = err;
        return;
    }

    // Collect all needed values before any mutation
    let name = app.wizard_state.name.trim().to_lowercase();
    let role = app.wizard_state.resolved_role();
    let specialties = app.wizard_state.resolved_specialties();
    let prompt = app.wizard_state.resolved_prompt();
    let root = app.root.clone();

    let meta = AgentMeta::new(&name, &role, specialties);
    let json = meta.to_json();

    let result = crate::brain::save_pending_agent(&root, &name, &json)
        .and_then(|_| crate::brain::save_pending_prompt(&root, &name, &prompt));

    match result {
        Ok(_) => {
            app.status_msg = format!(
                "Agent '{}' saved to _pending/ — run /treec-agents in Claude Code to activate",
                name
            );
            app.current_screen = super::app::Screen::Agents;
        }
        Err(e) => {
            app.wizard_state.error = format!("Failed to save: {}", e);
        }
    }
}

/// Render wizard overlay
pub fn render_wizard(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Dim background
    let bg = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(bg, area);

    // Centered popup
    let popup = centered_rect(70, 80, area);
    frame.render_widget(Clear, popup);

    let w = &app.wizard_state;
    let step_num = match w.step {
        WizardStep::Name => 1,
        WizardStep::Role => 2,
        WizardStep::Specialties => 3,
        WizardStep::Prompt => 4,
        WizardStep::Confirm => 5,
    };

    let title = format!(" Create Agent — Step {}/5 ", step_num);
    let outer = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    frame.render_widget(outer.clone(), popup);

    let inner = outer.inner(popup);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(inner);

    match w.step {
        WizardStep::Name => render_step_name(frame, chunks[0], w),
        WizardStep::Role => render_step_role(frame, chunks[0], w),
        WizardStep::Specialties => render_step_specialties(frame, chunks[0], w),
        WizardStep::Prompt => render_step_prompt(frame, chunks[0], w),
        WizardStep::Confirm => render_step_confirm(frame, chunks[0], w),
    }

    // Error / hint bar
    let hint = if w.error.is_empty() {
        step_hint(&w.step)
    } else {
        &w.error
    };
    let hint_style = if w.error.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Red)
    };
    let hint_widget = Paragraph::new(hint).style(hint_style);
    frame.render_widget(hint_widget, chunks[1]);
}

fn render_step_name(frame: &mut Frame, area: Rect, w: &WizardState) {
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Agent Name",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  Use lowercase and hyphens (e.g. backend-auth)"),
        Line::from(""),
        Line::from(vec![
            Span::raw("  > "),
            Span::styled(
                w.name.as_str(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("_", Style::default().fg(Color::Green)),
        ]),
    ];
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
}

fn render_step_role(frame: &mut Frame, area: Rect, w: &WizardState) {
    let items: Vec<ListItem> = ROLES
        .iter()
        .enumerate()
        .map(|(i, role)| {
            let style = if i == w.role_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let prefix = if i == w.role_index { "  * " } else { "  o " };
            ListItem::new(Line::from(Span::styled(
                format!("{}{}", prefix, role),
                style,
            )))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("  Select Role (up/down + Enter)")
            .borders(Borders::NONE),
    );
    frame.render_widget(list, area);
}

fn render_step_specialties(frame: &mut Frame, area: Rect, w: &WizardState) {
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Specialties",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  Separate with commas (e.g. Rust, JWT, REST API)"),
        Line::from(""),
        Line::from(vec![
            Span::raw("  > "),
            Span::styled(w.specialties.as_str(), Style::default().fg(Color::White)),
            Span::styled("_", Style::default().fg(Color::Green)),
        ]),
    ];
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
}

fn render_step_prompt(frame: &mut Frame, area: Rect, w: &WizardState) {
    let header = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Base Prompt",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  Describe this agent's behavior, focus and constraints"),
        Line::from(""),
    ]);

    let header_area = Rect {
        height: 4,
        ..area
    };
    let textarea_area = Rect {
        y: area.y + 4,
        height: area.height.saturating_sub(4),
        ..area
    };

    frame.render_widget(header, header_area);
    frame.render_widget(&w.prompt_textarea, textarea_area);
}

fn render_step_confirm(frame: &mut Frame, area: Rect, w: &WizardState) {
    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Confirm Agent Creation",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("  Name:         {}", w.name)),
        Line::from(format!("  Role:         {}", w.resolved_role())),
        Line::from(format!("  Specialties:  {}", w.specialties)),
        Line::from("  Prompt:       (saved to .prompt.md)"),
        Line::from(""),
        Line::from(Span::styled(
            "  Press Enter/Y to save  *  N to cancel",
            Style::default().fg(Color::Green),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  After saving, run /treec-agents in Claude Code to activate",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), area);
}

fn step_hint(step: &WizardStep) -> &'static str {
    match step {
        WizardStep::Name => "Type agent name  Enter: next  Esc: cancel",
        WizardStep::Role => "up/down: select  Enter: confirm  Esc: back",
        WizardStep::Specialties => {
            "Type specialties separated by commas  Enter: next  Esc: back"
        }
        WizardStep::Prompt => "Type base prompt  Ctrl+Enter: next  Esc: back",
        WizardStep::Confirm => "Enter/Y: save to _pending/  N: cancel  Esc: back",
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_state_initial() {
        let w = WizardState::new();
        assert_eq!(w.step, WizardStep::Name);
        assert!(w.name.is_empty());
    }

    #[test]
    fn test_wizard_validation_empty_name() {
        let w = WizardState::new();
        assert!(w.validate().is_some());
    }

    #[test]
    fn test_wizard_validation_spaces_in_name() {
        let mut w = WizardState::new();
        w.name = "my agent".to_string();
        assert!(w.validate().is_some());
    }

    #[test]
    fn test_wizard_validation_valid() {
        let mut w = WizardState::new();
        w.name = "my-agent".to_string();
        w.role_index = 0; // Backend Developer
        assert!(w.validate().is_none());
    }

    #[test]
    fn test_resolved_specialties() {
        let mut w = WizardState::new();
        w.specialties = "Rust, JWT , REST API".to_string();
        let specs = w.resolved_specialties();
        assert_eq!(specs, vec!["Rust", "JWT", "REST API"]);
    }
}
