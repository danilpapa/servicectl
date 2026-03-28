use std::process::Command;

pub fn run_services(services: &[String]) -> Result<(), String> {
    let mut cmd = build_command(services);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute docker: {e}"))?;

    if !status.success() {
        return Err("Docker command failed".into());
    }

    Ok(())
}

fn build_command(services: &[String]) -> Command {
    let mut cmd = Command::new("docker");
    cmd.args(["compose", "up", "--build", "-d"]);
    cmd.args(services);
    cmd
}