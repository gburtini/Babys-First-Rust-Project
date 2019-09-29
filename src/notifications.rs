extern crate libnotify;

pub fn send_notification(url: &str) {
    libnotify::init("rust-is-up").unwrap();
    let n = libnotify::Notification::new(url, Some("URL is down!"), None);
    n.set_urgency(libnotify::Urgency::Critical);
    n.show().unwrap();
    libnotify::uninit();
}
