use crate::configuration::MonitorConfiguration;
use crate::is_up::run_check;
use rayon::prelude::*;

pub fn run(configuration: MonitorConfiguration) {
    // TODO: add option to _maybe parallelize_ instead of always parallelizing.
    configuration.rules.par_iter().for_each(|monitor_rule| {
        run_check(monitor_rule);
    });
}
