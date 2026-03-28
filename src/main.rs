mod services;
mod models;
pub mod tui;
pub use tui::keyboard;

use std::{env, io};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, List, ListItem, ListState},
    layout::{Layout, Direction, Constraint},
    text::Span,
};
use crossterm::{
    event::{self},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use crate::keyboard::keyboard_actions::KeyAction;
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
            let result = keyboard::keyboard_interceptor::handle_event(
                &event::read()?,
                &mut state,
                &services,
                &mut selected,
            );
            match result {
                Ok(action) => {
                    match action {
                        KeyAction::Quit => break,
                        KeyAction::Enter(chosen) => {
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                            terminal.show_cursor()?;

                            for s in chosen {
                                println!("{s}");
                            }
                            return Ok(());
                        },
                        _ => {}
                    }
                },
                Err(e) => println!("keyboard error {}", e)
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}