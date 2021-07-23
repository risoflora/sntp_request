# `sntp_request`

[![Build Status][travis-badge]][travis-url]
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![License][license-badge]][license-url]

[travis-badge]: https://travis-ci.org/risoflora/sntp_request.svg
[travis-url]: https://travis-ci.org/risoflora/sntp_request
[crates-badge]: https://img.shields.io/crates/v/sntp_request.svg
[crates-url]: https://crates.io/crates/sntp_request
[docs-badge]: https://docs.rs/sntp_request/badge.svg
[docs-url]: https://docs.rs/sntp_request
[license-badge]: https://img.shields.io/crates/l/sntp_request.svg
[license-url]: https://github.com/risoflora/sntp_request#license

`sntp_request` Tiny Rust library to request timestamp from [NTP servers](http://www.ntp.org) through [SNTP protocol](https://tools.ietf.org/html/rfc4330).

## Example

The example below shows how to obtain precise timestamp from main NTP server:

```rust
extern crate chrono;
extern crate sntp_request;

use chrono::{Local, TimeZone};
use sntp_request::SntpRequest;

fn main() {
    let sntp = SntpRequest::new();
    let timestamp = Local.timestamp(sntp.get_unix_time().unwrap(), 0);
    println!("{}", timestamp);
}
```

Also, it is possible to get the raw timestamp, for example:

```rust
extern crate sntp_request;

use sntp_request::SntpRequest;

fn main() {
    let sntp = SntpRequest::new();
    let timestamp = sntp.get_raw_time().unwrap();
    let nsec = (timestamp.frac as f64 / u32::max_value() as f64) * 1000.0;
    println!("seconds: {} frac: {}", timestamp.secs, timestamp.frac);
    println!("milliseconds: {}", nsec);
}
```

## Usage

Add this to your `Cargo.toml`:

```ini
[dependencies]
sntp_request = "2.0.0"
```

and this to your crate root:

```rust
extern crate sntp_request;
```

## Contributions

Pull Requests and Issues are welcome!

## License

`sntp_request` is licensed under either of the following, at your option:

- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
