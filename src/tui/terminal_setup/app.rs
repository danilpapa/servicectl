use std::io;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::terminal_setup::Backend;

pub struct App {
    pub terminal: Terminal<Backend>,
}

impl App {
    pub fn setup() -> io::Result<Self> {
        enable_raw_mode()?;
        let stdout = io::stdout();
        execute!(&stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }
}