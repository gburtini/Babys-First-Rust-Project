use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

// use std::env;
// use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorRule {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorRules {
    pub rules: Vec<MonitorRule>,
}

pub fn load_configuration() -> MonitorRules {
    let json_rules = fs::read_to_string("monitor.json").expect("monitor.json could not be read.");
    let rules: MonitorRules =
        serde_json::from_str(&json_rules).expect("monitor.json could not be parsed.");

    rules
}