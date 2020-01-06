//! `sntp_request` Tiny Rust library to request timestamp from [NTP servers](http://www.ntp.org) trough [SNTP protocol](https://tools.ietf.org/html/rfc4330).
//!
//! # Example
//!
//! The example below shows how to obtain precise timestamp from main NTP server:
//!
//! ```rust
//! extern crate chrono;
//! extern crate sntp_request;
//!
//! use chrono::{Local, TimeZone};
//! use sntp_request::SntpRequest;
//!
//! fn main() {
//!     let sntp = SntpRequest::new();
//!     let timestamp = Local.timestamp(sntp.get_unix_time().unwrap(), 0);
//!     println!("{}", timestamp);
//! }
//! ```

use std::convert::TryInto;
use std::io::{self, Error, ErrorKind};
use std::mem;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

#[doc(hidden)]
#[macro_export]
macro_rules! read_be_u32 {
    ($input:expr) => {{
        let (int_bytes, rest) = $input.split_at(mem::size_of::<u32>());
        *$input = rest;
        u32::from_be_bytes(int_bytes.try_into().unwrap())
    }};
}

/// Default public NTP address.
pub const POOL_NTP_ADDR: &str = "pool.ntp.org:123";

/// SNTP object which holds the socket handle to obtain timestamp from NTP servers.
pub struct SntpRequest {
    socket: UdpSocket,
}

/// Specialized type for raw time result.
pub type RawTimeResult = io::Result<u32>;

/// Specialized type for Unix time result.
pub type UnixTimeResult = io::Result<i64>;

impl SntpRequest {
    /// Creates a new SNTP request object.
    pub fn new() -> SntpRequest {
        let sntp = SntpRequest {
            socket: UdpSocket::bind("0.0.0.0:0").unwrap(),
        };
        sntp.set_timeout(Duration::from_secs(5)).unwrap();
        sntp
    }

    /// Sets the inactivity time to the client get time out. If not specified, the client assumes 5 seconds as default.
    pub fn set_timeout(&self, timeout: Duration) -> io::Result<()> {
        let dur = Some(timeout);
        self.socket.set_write_timeout(dur)?;
        self.socket.set_read_timeout(dur)
    }

    /// Obtains the raw time from a NTP server address.
    pub fn get_raw_time_by_addr<A: ToSocketAddrs>(&self, addr: A) -> RawTimeResult {
        const BUF_SIZE: usize = 48;
        let mut buf = [0u8; BUF_SIZE];
        // header - 8 bit:
        // LI (2 bit) - 3 (not in sync), VN (3 bit) - 4 (version),
        // mode (3 bit) - 3 (client)
        buf[0] = (3 << 6) | (4 << 3) | 3;
        match self.socket.send_to(&buf, addr) {
            Ok(sent) => {
                if sent != BUF_SIZE {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid SNTP packet size sent",
                    ));
                }
            }
            Err(error) => return Err(error),
        }
        match self.socket.recv_from(&mut buf) {
            Ok((recv, _)) => {
                if recv != BUF_SIZE {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid SNTP packet size received",
                    ));
                }
                let hdr = buf[0];
                if (hdr & 0x38) >> 3 != 4 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Server returned wrong SNTP version",
                    ));
                }
                let mode = hdr & 0x7;
                if mode != 4 && mode != 5 {
                    return Err(Error::new(ErrorKind::Other, "Not a SNTP server reply"));
                }
                Ok(read_be_u32!(&mut &buf[40..44]))
            }
            Err(error) => return Err(error),
        }
    }

    /// Obtains the [Unix time](https://en.wikipedia.org/wiki/Unix_time) from a NTP server address.
    pub fn get_unix_time_by_addr<A: ToSocketAddrs>(&self, addr: A) -> UnixTimeResult {
        let raw_time = self.get_raw_time_by_addr(addr)?;
        Ok((raw_time - 2_208_988_800) as i64)
    }

    /// Obtains the raw time from default NTP server address [`POOL_NTP_ADDR`](constant.POOL_NTP_ADDR.html).
    pub fn get_raw_time(&self) -> RawTimeResult {
        self.get_raw_time_by_addr(POOL_NTP_ADDR)
    }

    /// Obtains the [Unix time](https://en.wikipedia.org/wiki/Unix_time) from default NTP server address [`POOL_NTP_ADDR`](constant.POOL_NTP_ADDR.html).
    pub fn get_unix_time(&self) -> UnixTimeResult {
        self.get_unix_time_by_addr(POOL_NTP_ADDR)
    }
}
