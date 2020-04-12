//! Basic example for how to obtain a raw NTP timestamp from main NTP server.

extern crate sntp_request;

use sntp_request::SntpRequest;

fn main() {
    let sntp = SntpRequest::new();
    let timestamp = sntp.get_raw_time().unwrap();
    let nsec = (timestamp.frac as f64 / u32::max_value() as f64) * 1000.0;
    println!("seconds: {} frac: {}", timestamp.secs, timestamp.frac);
    println!("milliseconds: {}", nsec);
}
