mod configuration;
mod is_up;
mod notifications;

use configuration::load_configuration;
use is_up::{is_up, RequestVerb};
use notifications::send_notification;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let url = &args[1];
    // if let Err(_) = Url::parse(url) {
    //     panic!("Invalid URL.");
    // };

    // TODO: add commander style code to add and remove configurations.
    // TODO: add tests.

    // load configurations in a file as JSON.
    let configuration = load_configuration();

    // TODO: parallelize multiple requests.
    for monitor_rule in configuration.rules.iter() {
        match is_up(&monitor_rule.url, RequestVerb::GET) {
            // TODO: handle bad status codes.
            Ok(res) => println!("{:#?} {:#?}", monitor_rule.url, res.elapsed),
            Err(_) => send_notification(&monitor_rule.url),
        }
    }
}
