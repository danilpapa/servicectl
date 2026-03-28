use std::fs;
use std::path::{Path, PathBuf};
use crate::models::compose::Compose;

pub fn parse_services(current_dir: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    let path = Path::new(current_dir)
        .join("docker-compose.yml");
    let container_content = fs::read_to_string(path)
        .expect("Error reading docker-compose.yml file");
    let yaml = serde_yaml::from_str::<Compose>(&container_content)
        .expect("Error parsing docker-compose.yml file");
    let mut services: Vec<String> = vec![];

    for service in yaml.services.keys() {
        services.push(service.clone());
    }
    Ok(services)
}
