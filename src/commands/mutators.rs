use crate::configuration::{mutate_configuration, MonitorRule, MutationRule, MutationRuleType};

pub fn add(config_path: &str, url: &str) {
    match mutate_configuration(
        config_path.to_string(),
        MutationRule {
            mutation: MutationRuleType::Add,
            rule: MonitorRule {
                url: url.to_string(),
            },
        },
    ) {
        Ok(_) => println!("Successfully added rule."),
        Err(e) => panic!(e),
    };
}

pub fn remove(config_path: &str, url: &str) {
    // TODO: add closest match with confirmation if it doesn't match.

    match mutate_configuration(
        config_path.to_string(),
        MutationRule {
            mutation: MutationRuleType::Remove,
            rule: MonitorRule {
                url: url.to_string(),
            },
        },
    ) {
        Ok(_) => println!("Successfully removed rule."),
        Err(e) => panic!(e),
    };
}
