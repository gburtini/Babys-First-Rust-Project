extern crate reqwest;

use std::io::Read;
use std::time::{Duration, Instant};

pub struct MonitorResult {
    pub elapsed: Duration,
    pub status: reqwest::StatusCode,
    pub body: String,
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
    let mut res = make_request(url, request_verb)?;
    let mut body = String::new();
    match res.read_to_string(&mut body) {
        Ok(res) => res,
        Err(_) => {
            return Err("Request made successfully, but result was not valid UTF-8.".to_string());
        }
    };

    // if it succeeded, return the time and any other metadata.
    return Ok(MonitorResult {
        elapsed: now.elapsed(),
        status: res.status(),
        body: body,
    });
}

use crate::notifications::send_notification;
use crate::configuration::MonitorRule;
pub fn run_check(rule: &MonitorRule) {
    match is_up(&rule.url, RequestVerb::GET) {
        // TODO: handle bad status codes.
        Ok(res) => println!("{:#?} {:#?}", rule.url, res.elapsed),
        Err(_) => send_notification(&rule.url),
    }
}
