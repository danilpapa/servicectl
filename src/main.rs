mod services;
mod models;
pub mod tui;
pub use tui::keyboard;
pub use tui::terminal_setup;
pub use tui::screen;

use std::env;
use std::fmt::Display;
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
use strum::IntoEnumIterator;
use crate::screen::AppScreen;
use crate::tui::options::ActionChoice;

fn main() -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let services = parse_services(&current_dir)
        .expect("Couldn't parse command line arguments");
    let mut app = terminal_setup::app::App::setup()?;
    let mut selected = vec![false; services.len()];
    let mut state = ListState::default();
    state.select(Some(0));
    let mut screen = AppScreen::SelectServices;
    let mut option = ActionChoice::BuildSelected;

    loop {
        match screen.clone() {
            AppScreen::SelectServices => {
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
                                    screen = AppScreen::Actions(chosen);
                                    state.select(Some(0));
                                },
                                _ => {}
                            }
                        },
                        Err(e) => println!("keyboard error {}", e)
                    }
                }
            },
            AppScreen::Actions(chosen) => {
                app.terminal.draw(|f| {
                    let size = f.area();

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),
                            Constraint::Min(1),
                        ])
                        .split(size);

                    let title = List::new(vec![ListItem::new(Span::raw("Select build options"))])
                        .block(Block::default().borders(Borders::ALL));
                    f.render_widget(title, chunks[0]);

                    let items: Vec<ListItem> = ActionChoice::iter()
                        .map(|action| {
                            ListItem::new(action.to_string())
                        })
                        .collect();

                    let list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("Options"))
                        .highlight_symbol(">> ");

                    f.render_stateful_widget(list, chunks[1], &mut state);
                })?;

                if event::poll(std::time::Duration::from_millis(200)).unwrap() {
                    let event = event::read()
                        .expect("Couldn't read keyboard event");
                    keyboard::keyboard_interceptor::handle_option(
                        &event,
                        &mut state,
                        &mut option,
                        &mut screen,
                        &chosen.clone()
                    );
                }
            }
        }
    }
    terminal_setup::terminal_setup::after_all(&mut app.terminal)?;
    Ok(())
}