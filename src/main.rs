mod services;
mod models;
pub mod tui;
pub use tui::keyboard;
pub use tui::terminal_setup;

use std::env;
use std::process::Command;
use ratatui::{
    widgets::{Block, Borders, List, ListItem, ListState},
    layout::{Layout, Direction, Constraint},
    text::Span,
};
use crossterm::{
    event::{self},
};
use crate::keyboard::keyboard_actions::KeyAction;
use crate::services::parse_compose::parse_services;
use crate::services::docker;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let services = parse_services(&args)
        .expect("Couldn't parse command line arguments");
    let mut app = terminal_setup::app::App::setup()?;
    let mut selected = vec![false; services.len()];
    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        app.terminal.draw(|f| {
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
                            terminal_setup::terminal_setup::tear_down(&mut app.terminal)?;
                            let mut execute: String = String::from("docker compose up --build -d ");
                            chosen
                                .iter()
                                .for_each(|service| {
                                    execute.push_str(service.as_str());
                                    execute.push(' ');
                                });

                            match docker::run_services(&chosen) {
                                Ok(_) => println!("Services started successfully"),
                                Err(e) => eprintln!("Error: {e}"),
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

    terminal_setup::terminal_setup::after_all(&mut app.terminal)?;
    Ok(())
}