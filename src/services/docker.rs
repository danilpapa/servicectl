use std::process::Command;
use ratatui::Terminal;
use crate::terminal_setup::Backend;
use crate::tui::options::ActionChoice;
pub use crate::tui::terminal_setup;

pub fn run_services(
    services: &[String],
    option: &ActionChoice,
    terminal: &mut Terminal<Backend>
) -> Result<(), String> {
    let mut cmd = build_command(services, &option);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute docker: {e}"))?;

    if !status.success() {
        return Err("Docker command failed".into());
    }
    clear_term();
    _ = terminal_setup::terminal_setup::after_all(terminal);
    Ok(())
}

fn build_command(
    services: &[String],
    option: &ActionChoice,
) -> Command {
    let mut cmd = Command::new("docker");
    cmd.args(["compose", "up", "--build", "-d"]);
    match option {
        ActionChoice::BuildSelected => {
            cmd.args(services);
        }
        _ => {}
    }
    cmd
}

fn clear_term() {
    let _ = Command::new("bash")
        .arg("-c")
        .arg("clear")
        .status();
}