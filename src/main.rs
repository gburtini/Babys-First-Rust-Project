#[macro_use]
extern crate prettytable;
extern crate rayon;

mod cli;
mod commands;
mod configuration;
mod is_up;
mod notifications;

use cli::build_cli;
use configuration::load_configuration;

fn main() {
    let app = build_cli();
    let matches = app.get_matches();

    let config_path = matches.value_of("config").unwrap_or("monitor.json");
    match matches.subcommand() {
        ("run", _) => {
            commands::run(load_configuration(config_path.to_string()));
        }

        ("once", Some(submatches)) => {
            let url = submatches.value_of("url").expect("No URL provided.");
            commands::once(url);
        }

        ("list", _) => {
            // pretty-print the blob of configured rules
            commands::list(load_configuration(config_path.to_string()));
        }

        ("report", _) => {
            // run the checks and output a CSV of response times and types.
            commands::report(load_configuration(config_path.to_string()));
        }
        ("start", Some(_submatches)) => {
            commands::start();
        }

        ("stop", Some(_submatches)) => {
            commands::stop();
        }

        ("add", Some(submatches)) => {
            // add a check to the checks file.
            let url = submatches.value_of("url").expect("No URL provided.");
            commands::add(config_path, url);
        }

        ("remove", Some(submatches)) => {
            // remove a check from the checks file by exact match
            let url = submatches.value_of("url").expect("No URL provided.");
            commands::remove(config_path, url);
        }

        ("generate-completions", Some(submatches)) => {
            let shell = submatches.value_of("shell").unwrap_or("bash");
            commands::generate_completions(shell);
        }

        (_, _) => {}
    };
}
