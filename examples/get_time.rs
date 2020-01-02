//! Basic example for how to obtain precise timestamp from main NTP server.

extern crate chrono;
extern crate sntp_request;

use chrono::{Local, TimeZone};
use sntp_request::SntpRequest;

fn main() {
    let sntp = SntpRequest::new();
    let timestamp = Local.timestamp(sntp.get_unix_time().unwrap(), 0);
    println!("{}", timestamp);
}
