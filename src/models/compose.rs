use std::collections::HashMap;
use serde::Deserialize;
use serde_yaml::Value;

#[derive(Deserialize)]
pub struct Compose {
    pub services: HashMap<String, Value>,
}