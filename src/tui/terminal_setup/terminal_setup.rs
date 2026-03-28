use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::{Terminal};
use std::io;
use crate::terminal_setup::Backend;

pub fn tear_down(terminal: &mut Terminal<Backend>) -> io::Result<()> {
    use crossterm::execute;
    use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn after_all(terminal: &mut Terminal<Backend>) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
