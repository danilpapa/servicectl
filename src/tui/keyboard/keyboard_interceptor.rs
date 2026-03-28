use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::widgets::ListState;
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
            KeyCode::Down => {
                if let Some(index) = state.selected() {
                    if index + 1 < services.len() {
                        state.select(Some(index + 1));
                        return Ok(Handled)
                    }
                }
                return Ok(KeyAction::None)
            },
            KeyCode::Up => {
                if let Some(index) = state.selected() {
                    if index - 1 < services.len() {
                        state.select(Some(index - 1));
                        return Ok(Handled)
                    }
                }
                return Ok(KeyAction::None)
            },
            KeyCode::Char(' ') => {
                if let Some(index) = state.selected() {
                    selected[index] = !selected[index];
                    return Ok(Handled)
                }
                return Ok(KeyAction::None)
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
            _ => {
                return Ok(KeyAction::None)
            }
        }
    }
    Err(String::from("Keyboard interceptor error"))
}