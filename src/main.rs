mod cli;
mod configuration;
mod is_up;
mod notifications;

use cli::build_cli;
use configuration::{load_configuration, MonitorRule};
use is_up::{is_up, RequestVerb};
use notifications::send_notification;

use clap::Shell;
use std::io;
use url::Url;

fn run_check(rule: &MonitorRule) {
    match is_up(&rule.url, RequestVerb::GET) {
        // TODO: handle bad status codes.
        Ok(res) => println!("{:#?} {:#?}", rule.url, res.elapsed),
        Err(_) => send_notification(&rule.url),
    }
}

fn main() {
    let app = build_cli();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("run", _) => {
            // load configurations in a file as JSON.
            let configuration = load_configuration();

            // TODO: parallelize multiple requests.
            for monitor_rule in configuration.rules.iter() {
                run_check(monitor_rule);
            }
        }

        ("once", Some(submatches)) => {
            // build a MonitorRule from the input.
            let url = submatches.value_of("url").expect("No URL provided.");
            let url = Url::parse(url).expect("Invalid URL provided.");
            // run one check.
            run_check(&MonitorRule {
                url: url.into_string(),
            });
        }

        ("list", _) => {
            // TODO: pretty-print the blob of configured rules
        }

        ("report", _) => {
            // TODO: run the checks and output a CSV of response times and types.
        }

        ("start", Some(submatches)) => {
            // TODO: start a daemon that will run the checks on a schedule.
        }

        ("stop", Some(submatches)) => {
            // TODO: stop that daemon.
        }

        ("add", Some(submatches)) => {
            // TODO: add a check to the checks file.
        }

        ("remove", Some(submatches)) => {
            // TODO: remove a check from the checks file by closest match with confirmation.
        }

        ("generate-completions", Some(submatches)) => {
            let shell = submatches.value_of("shell");
            let shell = match shell {
                Some("bash") => Shell::Bash,
                Some("fish") => Shell::Fish,
                Some("zsh") => Shell::Zsh,
                Some("powershell") => Shell::PowerShell,
                Some("elvish") => Shell::Elvish,
                _ => {
                    // TODO: infer the current shell?
                    panic!("Invalid shell provided to generate-completions.");
                }
            };

            build_cli().gen_completions_to("is-up", shell, &mut io::stdout());
        }

        (_, _) => {}
    };

    // TODO: add tests.
}
