use crate::configuration::{MonitorRule};
use crate::is_up::{run_check};

use url::Url;

pub fn once(url: &str) {
    let url = Url::parse(url).expect("Invalid URL provided.");
    // run one check.
    run_check(&MonitorRule {
        url: url.into_string(),
    });
}