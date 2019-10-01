use crate::cli::build_cli;
use clap::Shell;
use std::io;

pub fn generate_completions(shell: &str) {
    let shell = match shell {
        "bash" => Shell::Bash,
        "fish" => Shell::Fish,
        "zsh" => Shell::Zsh,
        "powershell" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        _ => {
            // TODO: infer the current shell?
            panic!("Invalid shell provided to generate-completions.");
        }
    };

    build_cli().gen_completions_to("is-up", shell, &mut io::stdout());
}