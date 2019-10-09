use std::fs;
use std::process::Command;

pub fn start() {
    let sleep_time = 100000; // millis, to be moved to config blob.
    Command::new("./is-up-daemon")
        .arg(sleep_time.to_string())
        .arg("is-up")
        .spawn()
        .expect("failed to spawn daemon process");
}

pub fn stop() {
    let pid = fs::read_to_string("/tmp/is-up.pid").expect("PID file (/tmp/is-up.pid) could not be read. It is likely the daemon is not running.");
    let pid = pid.parse::<u64>().expect(
        "PID was non-numeric. Cowardly refusing to try to kill a non-numeric process.",
    );

    if pid <= 1 {
        panic!("Invalid PID in /tmp/is-up.pid.");
    }

    Command::new("kill")
        .arg(pid.to_string())
        .output()
        .expect("Failed to kill process");
    fs::remove_file("/tmp/is-up.pid").expect(
        "Killed binary but could not remove /tmp/is-up.pid. Please remove manually.",
    );
}
