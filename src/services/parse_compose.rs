use std::fs;
use std::path::Path;
use crate::models::compose::Compose;

pub fn parse_services(args: &Vec<String>) -> Result<Vec<String>, std::io::Error> {
    if args.len() != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "❌ Недостаточно аргументов.\nИспользование: script /Desktop/Project/src")
        );
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
    Ok(services)
}
