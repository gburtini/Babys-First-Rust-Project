
use prettytable::Table;
use crate::configuration::MonitorConfiguration;

pub fn list(configuration: MonitorConfiguration) {
    let mut table = Table::new();

    // add a row for each URL.
    table.add_row(row!["URL"]);
    for monitor_rule in configuration.rules.iter() {
        table.add_row(row![monitor_rule.url]);
    }

    // print the table to stdout
    table.printstd();
    
}