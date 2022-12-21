//! Basic example for how to obtain precise timestamp from main NTP server.

use chrono::{Local, TimeZone};
use sntp_request::SntpRequest;

fn main() {
    let sntp = SntpRequest::new();
    let timestamp = Local.timestamp_opt(sntp.get_unix_time().unwrap(), 0);
    println!("{}", timestamp.unwrap());
}
