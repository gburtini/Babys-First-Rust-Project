use crate::is_up::{run_check};
use crate::configuration::{MonitorConfiguration};

pub fn run(configuration : MonitorConfiguration) {
    // TODO: parallelize multiple requests.
    for monitor_rule in configuration.rules.iter() {
        run_check(monitor_rule);
    }
}