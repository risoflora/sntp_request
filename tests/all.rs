extern crate sntp_request;

use std::thread;
use std::time::Duration;

use sntp_request::SntpRequest;

#[test]
fn get_unix_time() {
    let sntp = SntpRequest::new();
    sntp.set_timeout(Duration::from_secs(10)).unwrap();
    let t1 = sntp.get_unix_time().unwrap();
    thread::sleep(Duration::from_secs(2));
    let t2 = sntp.get_unix_time().unwrap();
    assert!(t2 > t1);
}
