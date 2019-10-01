use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorRule {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorConfiguration {
    pub rules: Vec<MonitorRule>,
}

pub fn load_configuration(path: String) -> MonitorConfiguration {
    let json_rules = fs::read_to_string(path).expect("Configuration file could not be read.");
    let rules: MonitorConfiguration =
        serde_json::from_str(&json_rules).expect("Configuration file could not be parsed.");

    rules
}
