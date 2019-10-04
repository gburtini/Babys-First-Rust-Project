extern crate reqwest;

use std::io::Read;
use std::time::{Duration, Instant};

pub struct MonitorResult {
    pub elapsed: Duration,
    pub status: reqwest::StatusCode,
    pub body: Option<String>,
}

pub enum RequestVerb {
    GET,
    // POST,
    // HEAD,
    // DELETE,
    // PUT,
    // OPTIONS
}

fn handler(e: reqwest::Error) -> String {
    if e.is_http() {
        if let None = e.url() {
            return "No URL given.".to_string();
        }
    }

    if e.is_serialization() {
        return "Internal error with request.".to_string();
    }

    if e.is_redirect() {
        return "Failed with a redirect in place -- perhaps server is redirecting too many times."
            .to_string();
    }

    return "An unknown error has occured.".to_string();
}

fn make_request(url: &str, request_verb: RequestVerb) -> Result<reqwest::Response, String> {
    let requester = match request_verb {
        RequestVerb::GET => reqwest::get,
    };

    return match requester(url) {
        Ok(res) => Ok(res),
        Err(e) => Err(handler(e)),
    };
}

pub fn is_up(url: &str, request_verb: RequestVerb) -> Result<MonitorResult, String> {
    let now = Instant::now();

    // make network request to URL, if it fails, return immediately.
    let res = make_request(url, request_verb)?;
    // if it succeeded, return the time and any other metadata.
    return Ok(MonitorResult {
        elapsed: now.elapsed(),
        status: res.status(),
        body: None,
    });
}

// TODO: it feels to me like there's a better way to compose these things
// as it stands, there's a lot of duplication across the sub-implementations.
pub fn is_up_and_status_ok(url: &str, request_verb: RequestVerb) -> Result<MonitorResult, String> {
    let res = is_up(url, request_verb)?;
    match res.status.as_u16() {
        // handle bad status codes better.
        200..=299 => Ok(res),
        _ => Err("Status was ".to_string() + res.status.as_str()),
    }
}

pub fn is_up_with_body(url: &str, request_verb: RequestVerb) -> Result<MonitorResult, String> {
    let now = Instant::now();

    // make network request to URL, if it fails, return immediately.
    let mut res = make_request(url, request_verb)?;
    let mut body = String::new();
    match res.read_to_string(&mut body) {
        Ok(res) => res,
        Err(e) => {
            println!("{:#}", e);
            return Err("Request made successfully, but result was not valid UTF-8.".to_string());
        }
    };

    // if it succeeded, return the time and any other metadata.
    return Ok(MonitorResult {
        elapsed: now.elapsed(),
        status: res.status(),
        body: Some(body),
    });
}

use crate::configuration::MonitorRule;
use crate::notifications::send_notification;
pub fn run_check(rule: &MonitorRule) {
    match is_up_and_status_ok(&rule.url, RequestVerb::GET) {
        Ok(res) => {
            println!("{:#?} {:#?}", rule.url, res.elapsed);
        }
        Err(_) => send_notification(&rule.url),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn google_is_up() -> Result<(), String> {
        match super::is_up("https://www.google.ca/", super::RequestVerb::GET) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[test]
    fn crates_is_up() -> Result<(), String> {
        match super::is_up_with_body("https://crates.io/", super::RequestVerb::GET) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[test]
    fn example_404s_properly() -> Result<(), String> {
        match super::is_up_and_status_ok("https://example.com/404", super::RequestVerb::GET) {
            Ok(_) => Err("Should have 404'd.".to_string()),
            Err(_) => Ok(()),
        }
    }

}
