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
use std::fs;
use std::process::Command;

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
            let sleep_time = 100000; // millis
            Command::new("./is-up-daemon")
                .arg(sleep_time.to_string())
                .arg("is-up")
                .spawn()
                .expect("failed to spawn daemon process");
        }

        ("stop", Some(_submatches)) => {
            let pid = fs::read_to_string("/tmp/is-up.pid").expect("PID file (/tmp/is-up.pid) could not be read. It is likely the daemon is not running.");
            // TODO: validate format of PID?
            Command::new("kill")
                .arg(pid)
                .output()
                .expect("failed to kill");
            fs::remove_file("/tmp/is-up.pid").expect(
                "Killed binary but could not remove /tmp/is-up.pid. Please remove manually.",
            );
        }

        ("add", Some(submatches)) => {
            // add a check to the checks file.
            // TODO: these guys are broken because they are unable to write to the file.
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
