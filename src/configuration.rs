use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MonitorRule {
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MonitorConfiguration {
    pub rules: Vec<MonitorRule>,
}

pub fn load_configuration(path: String) -> MonitorConfiguration {
    let json_rules = fs::read_to_string(path).expect("Configuration file could not be read.");
    let rules: MonitorConfiguration =
        serde_json::from_str(&json_rules).expect("Configuration file could not be parsed.");

    rules
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MutationRuleType {
    Add,
    Remove,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MutationRule {
    pub mutation: MutationRuleType,
    pub rule: MonitorRule,
}

pub fn mutate_configuration(
    path: String,
    change: MutationRule,
) -> Result<MonitorConfiguration, String> {
    let original_rules = load_configuration(path.clone());
    let new_rules = mutate_configuration_from_rules(original_rules, change)?;
    return write_configuration(path, new_rules);
}

pub fn find_rule_from_rules(
    current_rules: &MonitorConfiguration,
    url: String,
) -> Result<usize, usize> {
    return current_rules
        .rules
        .binary_search_by_key(&url, |rule| rule.url.to_string());
}
pub fn mutate_configuration_from_rules(
    current_rules: MonitorConfiguration,
    change: MutationRule,
) -> Result<MonitorConfiguration, String> {
    let mut new_rules = current_rules.clone();

    match change.mutation {
        MutationRuleType::Add => {
            new_rules.rules.push(change.rule);
        }
        MutationRuleType::Remove => {
            let rule_index = find_rule_from_rules(&current_rules, change.rule.url.to_string())
                .expect("No rule matched");

            new_rules.rules.remove(rule_index);
        }
    }

    return Ok(new_rules);
}
pub fn write_configuration(
    path: String,
    config: MonitorConfiguration,
) -> Result<MonitorConfiguration, String> {
    let rules =
        serde_json::to_string(&config).expect("Configuration could not be serialized to JSON.");
    let mut file =
        fs::File::create(path).expect("Could not open specified configuration file for updating");
    match file.write_all(rules.as_bytes()) {
        Ok(_) => return Ok(config),
        Err(e) => {
            return Err(format!(
                "Failed to write to specified configuration file ({})",
                e.to_string()
            ))
        }
    }
}
