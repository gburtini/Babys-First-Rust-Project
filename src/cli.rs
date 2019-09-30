use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    return App::new("is-up?")
        .version("1.0")
        .author("Giuseppe Burtini <giuseppe@burtini.ca>")
        .about("Monitors server uptime.")
        .arg(
            Arg::with_name("config")
                .help("Sets the configuration file to use (default: monitor.json)")
                .short("c")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity (repeat v for more verbose)"),
        )
        .subcommand(
            SubCommand::with_name("once").about("Run a check on a single URL")
            .arg(Arg::with_name("url").help("The URL to be tested once."))
        )
        .subcommand(SubCommand::with_name("run").about("Run the configured checks"))
        .subcommand(SubCommand::with_name("add").about("Add a check to the configuration"))
        .subcommand(SubCommand::with_name("remove").about("Remove a configured check"))
        .subcommand(SubCommand::with_name("list").about("List all configured checks"))
        .subcommand(SubCommand::with_name("report").about("Produce a CSV response report"))
        .subcommand(
            SubCommand::with_name("generate-completions").about("Generates completions, e.g., `is-up generate-bash-completions > /usr/share/bash-completion/completions/is-up.bash`")
            .arg(Arg::with_name("shell").help("One of bash, fish, zsh, powershell or elvish."))
        );
}
