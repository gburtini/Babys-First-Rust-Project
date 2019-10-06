use daemonize::Daemonize;
use std::env;
use std::fs::File;
use std::process::Command;
use std::{thread, time};

fn main() {
    let stdout = File::create("/tmp/is-up.out").unwrap();
    let stderr = File::create("/tmp/is-up.err").unwrap();
    let daemonize = Daemonize::new()
        .pid_file("/tmp/is-up.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .user("nobody")
        .group("daemon")
        .group(2)
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .exit_action(|| println!("Forking daemon process. All further output in log files."))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            let args: Vec<String> = env::args().collect();

            let sleep_time = &args[1]
                .parse::<u64>()
                .expect("Expected a valid sleep time.");

            let parent_binary = &args[2];

            loop {
                // TODO: make sure there's no currently running check to prevent becoming a fork bomb.
                // run checks.
                let _output = Command::new(parent_binary)
                    .arg("run")
                    .output()
                    .expect("failed to execute check process");

                // handle output.

                // sleep for configured amount of time.
                thread::sleep(time::Duration::from_millis(*sleep_time));
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
