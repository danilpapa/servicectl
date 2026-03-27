use std::collections::HashMap;
use std::{fs};
use std::path::{Path};
use serde::Deserialize;
use serde_yaml::Value;

#[derive(Deserialize)]
struct Compose {
    services: HashMap<String, Value>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("❌ Недостаточно аргументов.\nИспользование: script /Desktop/Project/src");
    }

    let path = Path::new(&args[1])
        .join("docker-compose.yml");
    let container_content = fs::read_to_string(path)
        .expect("Error reading docker-compose.yml file");
    let yaml = serde_yaml::from_str::<Compose>(&container_content)
        .expect("Error parsing docker-compose.yml file");
    let mut services: Vec<String> = vec![];

    for service in yaml.services.keys() {
        services.push(service.clone());
    }
}
