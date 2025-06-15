use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

use crate::config::Config;

pub fn run_tui(config_path: &str, mut config: Config) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut config, config_path);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn draw_ui(
    f: &mut Frame,
    config: &Config,
    items: &[&str],
    size: ratatui::layout::Size,
) {
    let min_height = 6; // Minimum: 1 for list, 2 for config, 3 for footer
    let constraints = if size.height < min_height {
        vec![
            Constraint::Min(1),    // List
            Constraint::Length(2), // Config
            Constraint::Length(3), // Footer with warning
        ]
    } else {
        vec![
            Constraint::Min(3),    // List
            Constraint::Length(2), // Config
            Constraint::Length(3), // Footer
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.area());

    let block = Block::default()
        .title("RustReady TUI Core")
        .borders(Borders::ALL);
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|&i| ListItem::new(Span::raw(i)))
        .collect();
    let list = List::new(list_items)
        .block(block)
        .style(Style::default().fg(Color::White));
    f.render_widget(list, chunks[0]);

    let config_text = Paragraph::new(Line::from(vec![
        Span::styled("Last Run: ", Style::default().fg(Color::Yellow)),
        Span::raw(config.last_run.trim()),
    ]))
    .block(Block::default().borders(Borders::TOP));
    f.render_widget(config_text, chunks[1]);

    let footer_text = if size.height < min_height {
        "Terminal too small! Resize or press 'q' to quit"
    } else {
        "Press 'q' to quit, 'r' to reset config"
    };
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(footer, chunks[2]);
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    config: &mut Config,
    config_path: &str,
) -> Result<()> {
    let items = vec!["Item 1", "Item 2", "Item 3"];
    loop {
        let size = terminal.size()?;

        terminal.draw(|f| {
            draw_ui(f, config, &items, size);
        })?;

        match event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
                if key.code == KeyCode::Char('r') {
                    config.last_run = "reset".to_string();
                    config.save(config_path)?;
                    let size = terminal.size()?;
                    terminal.draw(|f| {
                        draw_ui(f, config, &items, size);
                    })?;
                }
            }
            Event::Resize(_, _) => {
                let size = terminal.size()?;
                terminal.draw(|f| {
                    draw_ui(f, config, &items, size);
                })?;
            }
            _ => {}
        }
    }
}
