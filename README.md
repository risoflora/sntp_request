# `sntp_request`

[![CI/CD][ci-cd-badge]][ci-cd-url]
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![License][license-badge]][license-url]

`sntp_request` Tiny Rust library to request timestamps from [NTP servers](http://www.ntp.org) through [SNTP protocol](https://tools.ietf.org/html/rfc4330).

## Example

The example below shows how to obtain precise timestamp from main NTP server:

```rust
use chrono::{Local, TimeZone};
use sntp_request::SntpRequest;

fn main() {
    let sntp = SntpRequest::new();
    let timestamp = Local.timestamp_opt(sntp.get_unix_time().unwrap(), 0);
    println!("{}", timestamp.unwrap());
}
```

Also, it is possible to get the raw timestamp, for example:

```rust
use sntp_request::SntpRequest;

fn main() {
    let sntp = SntpRequest::new();
    let timestamp = sntp.get_raw_time().unwrap();
    let nsec = (timestamp.frac as f64 / u32::max_value() as f64) * 1000.0;
    println!("seconds: {} frac: {}", timestamp.secs, timestamp.frac);
    println!("milliseconds: {}", nsec);
}
```

## Contributions

Pull Requests are welcome! =)

## License

`sntp_request` is licensed under the [Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT).

[ci-cd-badge]: https://img.shields.io/github/actions/workflow/status/risoflora/sntp_request/CI.yml?branch=main
[ci-cd-url]: https://github.com/risoflora/sntp_request/actions/workflows/CI.yml
[crates-badge]: https://img.shields.io/crates/v/sntp_request.svg
[crates-url]: https://crates.io/crates/sntp_request
[docs-badge]: https://docs.rs/sntp_request/badge.svg
[docs-url]: https://docs.rs/sntp_request
[license-badge]: https://img.shields.io/crates/l/sntp_request.svg
[license-url]: https://github.com/risoflora/sntp_request#license
