#[macro_use]
extern crate prettytable;
extern crate rayon;

mod cli;
mod commands;
mod configuration;
mod is_up;
mod notifications;

use cli::build_cli;
use configuration::{
    load_configuration, mutate_configuration, MonitorRule, MutationRule, MutationRuleType,
};

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
            // TODO: start a daemon that will run the checks on a schedule.

            // This probably isn't in the right place because:
            // > Yes, in the process of daemonizing, the original (foreground) process exits, and a copy of it runs in the background.
            // let daemonize = Daemonize::new()
            //     .pid_file("/tmp/test.pid") // Every method except `new` and `start`
            //     .chown_pid_file(true)      // is optional, see `Daemonize` documentation
            //     .working_directory("/tmp") // for default behaviour.
            //     .user("nobody")
            //     .group("daemon") // Group name
            //     .group(2)        // or group id.
            //     .umask(0o777)    // Set umask, `0o027` by default.
            //     .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
            //     .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
            //     .exit_action(|| println!("Executed before master process exits"))
            //     .privileged_action(|| "Executed before drop privileges");

            // match daemonize.start() {
            //     Ok(_) => println!("Success, daemonized"),
            //     Err(e) => eprintln!("Error, {}", e),
            // }
        }

        ("stop", Some(_submatches)) => {
            // TODO: stop that daemon.
        }

        ("add", Some(submatches)) => {
            // add a check to the checks file.
            let url = submatches.value_of("url").expect("No URL provided.");
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

        ("remove", Some(submatches)) => {
            // remove a check from the checks file by exact match 
            // TODO: add closest match with confirmation
            
            let url = submatches.value_of("url").expect("No URL provided.");
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

        ("generate-completions", Some(submatches)) => {
            let shell = submatches.value_of("shell").unwrap_or("bash");
            commands::generate_completions(shell);
        }

        (_, _) => {}
    };
}
