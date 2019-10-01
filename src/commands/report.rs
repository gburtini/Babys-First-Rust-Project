use crate::is_up::{is_up, RequestVerb};
use crate::configuration::{MonitorConfiguration};
use csv::Writer;

pub fn report(configuration : MonitorConfiguration) {
    let mut writer = Writer::from_writer(vec![]);
    writer
        .write_record(&["url", "method", "seconds", "status", "bytes"])
        .ok()
        .expect("CSV writer error");
    for monitor_rule in configuration.rules.iter() {
        // TODO: need to figure out how to represent the response type still.
        let result = match is_up(&monitor_rule.url, RequestVerb::GET) {
            Ok(result) => [
                String::from(&monitor_rule.url),
                String::from("GET"), // TODO: figure out how to serialize an ENUM elegantly.
                result.elapsed.as_secs().to_string(),
                result.status.to_string(),
                result.body.len().to_string() + " bytes",
            ],
            Err(err) => [
                String::from(&monitor_rule.url),
                String::from("GET"),
                String::from("-"),
                String::from("-"),
                err,
            ],
        };

        writer.write_record(&result).ok().expect("CSV writer error");
    }
    println!(
        "{:?}",
        String::from_utf8(writer.into_inner().expect("CSV writer error"))
    );
}