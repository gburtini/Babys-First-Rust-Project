extern crate libnotify;

mod is_up;

use is_up::{is_up, RequestVerb};
use std::env;

fn send_notification(url: &str) {
    libnotify::init("rust-is-up").unwrap();
    let n = libnotify::Notification::new(url, Some("URL is down!"), None);
    n.set_urgency(libnotify::Urgency::Critical);
    n.show().unwrap();
    libnotify::uninit();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: store configurations in a file.

    let url = &args[1];
    // TODO: validation and such.

    match is_up(url, RequestVerb::GET) {
        Ok(res) => println!("{:#?}", res.elapsed),
        Err(_) => send_notification(url),
    }
}
