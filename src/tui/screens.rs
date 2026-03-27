use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use super::app::{App, Screen};
use super::wizard;

/// Main render dispatch
pub fn render(frame: &mut Frame, app: &App) {
    if app.current_screen == Screen::CreateAgent {
        wizard::render_wizard(frame, app);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title bar
            Constraint::Min(0),    // content
            Constraint::Length(1), // status bar
        ])
        .split(frame.area());

    render_title_bar(frame, chunks[0], app);
    render_content(frame, chunks[1], app);
    render_status_bar(frame, chunks[2], app);
}

fn render_title_bar(frame: &mut Frame, area: Rect, app: &App) {
    let nav = Screen::all_nav();
    let titles: Vec<Span> = nav.iter().enumerate().map(|(i, s)| {
        let style = if app.nav_index == i {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        Span::styled(format!(" {} ", s.label()), style)
    }).collect();

    let title = format!(" TreeC v{} ", env!("CARGO_PKG_VERSION"));
    let title_bar = Paragraph::new(Line::from(titles))
        .block(Block::default()
            .title(title.as_str())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)));

    frame.render_widget(title_bar, area);
}

fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    match &app.current_screen {
        Screen::Dashboard => render_dashboard(frame, area, app),
        Screen::Agents => render_agents(frame, area, app),
        Screen::Tasks => render_tasks(frame, area, app),
        Screen::BrainViewer => render_brain_viewer(frame, area, app),
        Screen::SharedMemory => render_shared_memory(frame, area, app),
        Screen::Changelog => render_changelog(frame, area, app),
        Screen::CreateAgent => {} // handled above
    }
}

fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let msg = if app.status_msg.is_empty() {
        "Tab/Shift+Tab: navigate  up/down/jk: scroll  q: quit  n: new agent (on Agents screen)"
    } else {
        &app.status_msg
    };
    let status = Paragraph::new(msg)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(status, area);
}

// -- Screen: Dashboard ---------------------------------------------------------

fn render_dashboard(frame: &mut Frame, area: Rect, app: &App) {
    let brain_dir = app.root.join(".brain");
    let exists = brain_dir.exists();

    let agents_count = if brain_dir.join("agents").exists() {
        std::fs::read_dir(brain_dir.join("agents"))
            .ok()
            .map(|e| e.flatten().filter(|e| e.path().is_dir() && !e.file_name().to_string_lossy().starts_with('_')).count())
            .unwrap_or(0)
    } else { 0 };

    let tasks_open = read_brain_file_count(&app.root, "orchestrator/tasks.md", "- [ ]");
    let changelog_entries = read_brain_file_count(&app.root, "shared_memory/changelog.md", "##");

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled("  TreeC Neural Brain — Dashboard", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(format!("  Brain:       {}", if exists { "Initialized" } else { "Not found — run treec --neural-link" })),
        Line::from(format!("  Agents:      {} scaffolded", agents_count)),
        Line::from(format!("  Open Tasks:  {}", tasks_open)),
        Line::from(format!("  Changelog:   {} entries", changelog_entries)),
        Line::from(""),
        Line::from(Span::styled("  Shortcuts", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("  1-6       Navigate screens"),
        Line::from("  n         Create new agent (on Agents screen)"),
        Line::from("  q / Esc   Quit"),
    ];

    let block = Block::default().title(" Dashboard ").borders(Borders::ALL);
    let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });
    frame.render_widget(para, area);
}

// -- Screen: Agents ------------------------------------------------------------

fn render_agents(frame: &mut Frame, area: Rect, app: &App) {
    let agents_dir = app.root.join(".brain").join("agents");
    let mut items: Vec<ListItem> = vec![];

    if agents_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&agents_dir) {
            let mut names: Vec<String> = entries
                .flatten()
                .filter_map(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    if e.path().is_dir() && !name.starts_with('_') { Some(name) } else { None }
                })
                .collect();
            names.sort();

            for name in &names {
                let tasks = read_brain_file_count(&app.root, &format!("agents/{}/tasks.md", name), "- [ ]");
                let role = read_first_role_line(&app.root, &format!("agents/{}/identity.md", name));
                items.push(ListItem::new(Line::from(vec![
                    Span::styled(format!("  {:<20}", name), Style::default().fg(Color::Cyan)),
                    Span::styled(format!("{:<30}", role), Style::default().fg(Color::Gray)),
                    Span::styled(format!("{} tasks", tasks), Style::default().fg(Color::Yellow)),
                ])));
            }
        }
    }

    let pending = crate::brain::list_agents(&app.root, "_pending");
    for name in &pending {
        items.push(ListItem::new(Line::from(vec![
            Span::styled(format!("  {:<20}", name), Style::default().fg(Color::DarkGray)),
            Span::styled("(pending — awaiting skill activation)", Style::default().fg(Color::DarkGray)),
        ])));
    }

    if items.is_empty() {
        items.push(ListItem::new("  (no agents — press 'n' to create one)"));
    }

    let list = List::new(items)
        .block(Block::default().title(" Agents   [n] Create new ").borders(Borders::ALL));
    frame.render_widget(list, area);
}

// -- Screen: Tasks -------------------------------------------------------------

fn render_tasks(frame: &mut Frame, area: Rect, app: &App) {
    let content = read_brain_or_placeholder(
        &app.root,
        "orchestrator/tasks.md",
        "(no tasks — populate orchestrator/tasks.md via skill)",
    );
    let para = Paragraph::new(content)
        .block(Block::default().title(" Orchestrator Tasks ").borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    frame.render_widget(para, area);
}

// -- Screen: Brain Viewer ------------------------------------------------------

fn render_brain_viewer(frame: &mut Frame, area: Rect, app: &App) {
    let title = format!(" {} ", app.viewer_title);
    let para = Paragraph::new(app.viewer_content.as_str())
        .block(Block::default().title(title).borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    frame.render_widget(para, area);
}

// -- Screen: Shared Memory -----------------------------------------------------

fn render_shared_memory(frame: &mut Frame, area: Rect, app: &App) {
    let content = read_brain_or_placeholder(
        &app.root,
        "shared_memory/knowledge.md",
        "(shared memory not initialized)",
    );
    let para = Paragraph::new(content)
        .block(Block::default().title(" Shared Memory ").borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    frame.render_widget(para, area);
}

// -- Screen: Changelog ---------------------------------------------------------

fn render_changelog(frame: &mut Frame, area: Rect, app: &App) {
    let content = read_brain_or_placeholder(
        &app.root,
        "shared_memory/changelog.md",
        "(changelog empty)",
    );
    let para = Paragraph::new(content)
        .block(Block::default().title(" Changelog ").borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    frame.render_widget(para, area);
}

// -- Helpers -------------------------------------------------------------------

fn read_brain_or_placeholder(root: &std::path::Path, rel: &str, placeholder: &str) -> String {
    let path = root.join(".brain").join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|_| placeholder.to_string())
}

fn read_brain_file_count(root: &std::path::Path, rel: &str, pattern: &str) -> usize {
    let path = root.join(".brain").join(rel);
    std::fs::read_to_string(&path)
        .map(|c| c.lines().filter(|l| l.contains(pattern)).count())
        .unwrap_or(0)
}

fn read_first_role_line(root: &std::path::Path, rel: &str) -> String {
    let path = root.join(".brain").join(rel);
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|c| c.lines().find(|l| l.contains("**Role:**")).map(|l| {
            l.replace("**Role:**", "").trim().to_string()
        }))
        .unwrap_or_else(|| "Unknown".to_string())
}
