use std::process::Command;
use crate::tui::options::ActionChoice;

pub fn run_services(
    services: &[String],
    option: &ActionChoice
) -> Result<(), String> {
    let mut cmd = build_command(services, &option);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute docker: {e}"))?;

    if !status.success() {
        return Err("Docker command failed".into());
    }

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