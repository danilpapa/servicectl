use std::io::Stdout;
use ratatui::backend::CrosstermBackend;

pub mod terminal_setup;
pub mod app;

pub type Backend = CrosstermBackend<Stdout>;
