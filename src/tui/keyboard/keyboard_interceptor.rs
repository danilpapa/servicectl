use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::widgets::{ListState};
use crate::{services, ActionChoice};
use crate::screen::AppScreen;
use crate::tui::keyboard::keyboard_actions::KeyAction;
use crate::tui::keyboard::keyboard_actions::KeyAction::{Handled, Quit};

pub fn handle_event(
    event: &Event,
    state: &mut ListState,
    services: &Vec<String>,
    selected: &mut Vec<bool>,
) -> Result<KeyAction, String> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                return Ok(Quit);
            }
            KeyCode::Esc => {
                return Ok(Quit);
            }
            KeyCode::Down => {
                if let Some(index) = state.selected() {
                    if index + 1 < services.len() {
                        state.select(Some(index + 1));
                        return Ok(Handled)
                    }
                }
            },
            KeyCode::Up => {
                if let Some(index) = state.selected() {
                    let new_index = index.saturating_sub(1);
                    state.select(Some(new_index));
                    return Ok(Handled);
                }
            },
            KeyCode::Char(' ') => {
                if let Some(index) = state.selected() {
                    selected[index] = !selected[index];
                    return Ok(Handled)
                }
            },
            KeyCode::Enter => {
                let chosen: Vec<String> = services
                    .iter()
                    .enumerate()
                    .filter(|(position, _)| selected[*position])
                    .map(|(_, service)| service.clone())
                    .collect();
                return Ok(KeyAction::Enter(chosen));
            }
            _ => {}
        }
    }
    Ok(KeyAction::None)
}

pub fn handle_option(
    event: &Event,
    state: &mut ListState,
    option: &mut ActionChoice,
    screen: &mut AppScreen,
    services: &Vec<String>,
) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Down | KeyCode::Up => {
                option.toggle();
                state.select(Some(option.to_index()));
            },
            KeyCode::Esc => {
                *screen = AppScreen::SelectServices;
            },
            KeyCode::Enter => {
                _ = services::docker::run_services(&services, &option);
            },
            _ => {}
        }
    }
}