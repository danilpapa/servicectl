mod services;
mod models;
use std::{env, io};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, List, ListItem, ListState},
    layout::{Layout, Direction, Constraint},
    text::Span,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use crate::services::parse_compose::parse_services;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let services = parse_services(&args)
        .expect("Couldn't parse command line arguments");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected = vec![false; services.len()];
    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                ])
                .split(size);

            let title = List::new(vec![ListItem::new(Span::raw("Выбери сервисы которые надо пересобрать"))])
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            let items: Vec<ListItem> = services
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let mark = if selected[i] { "[x]" } else { "[ ]" };
                    ListItem::new(format!("{mark} {s}"))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Services"))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[1], &mut state);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        if let Some(i) = state.selected() {
                            if i + 1 < services.len() {
                                state.select(Some(i + 1));
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(i) = state.selected() {
                            if i > 0 {
                                state.select(Some(i - 1));
                            }
                        }
                    }
                    KeyCode::Char(' ') => {
                        if let Some(i) = state.selected() {
                            selected[i] = !selected[i];
                        }
                    }
                    KeyCode::Enter => {
                        let chosen: Vec<_> = services
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| selected[*i])
                            .map(|(_, s)| s.clone())
                            .collect();

                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        terminal.show_cursor()?;

                        println!("Selected services:");
                        for s in chosen {
                            println!("{s}");
                        }
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}