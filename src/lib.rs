//! `sntp_request` Tiny Rust library to request timestamp from [NTP servers](http://www.ntp.org) trough [SNTP protocol](https://tools.ietf.org/html/rfc4330).
//!
//! # Example
//!
//! The example below shows how to obtain precise timestamp from main NTP server:
//!
//! ```rust
//! use chrono::{Local, TimeZone};
//! use sntp_request::SntpRequest;
//!
//! let sntp = SntpRequest::new();
//! let timestamp = Local.timestamp_opt(sntp.get_unix_time().unwrap(), 0);
//! println!("{}", timestamp.unwrap());
//! ```

use std::cell::Cell;
use std::convert::TryInto;
use std::io::{self, Error, ErrorKind};
use std::mem;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

const SNTP_TIME_OFFSET: u32 = 2_208_988_800;

const SNTP_PACKET_SIZE: usize = 48;

#[inline]
fn read_be_u32(input: &[u8]) -> u32 {
    let (int_bytes, _) = input.split_at(mem::size_of::<u32>());
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

/// Default public NTP address.
pub const POOL_NTP_ADDR: &str = "pool.ntp.org:123";

/// SNTP object which holds the socket handle to obtain timestamp from NTP servers.
pub struct SntpRequest {
    socket: UdpSocket,
    kiss_of_death: Cell<bool>,
}

/// SNTP timestamp.
pub struct SntpTimestamp {
    /// Seconds since era epoch.
    pub secs: u32,
    /// Fraction of second.
    pub frac: u32,
}

/// Specialized type for raw time result.
pub type SntpRawTimeResult = io::Result<SntpTimestamp>;

/// Specialized type for Unix time result.
pub type SntpUnixTimeResult = io::Result<i64>;

impl Default for SntpRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl SntpRequest {
    /// Creates a new SNTP request object.
    pub fn new() -> SntpRequest {
        let sntp = SntpRequest {
            socket: UdpSocket::bind("0.0.0.0:0").unwrap(),
            kiss_of_death: Cell::new(false),
        };
        sntp.set_timeout(Duration::from_secs(5)).unwrap();
        sntp
    }

    #[inline]
    fn send_packet<A: ToSocketAddrs>(&self, addr: A, packet: &mut [u8]) -> io::Result<u32> {
        // LI (2 bit) - 3 (not in sync), VN (3 bit) - 4 (version),
        // mode (3 bit) - 3 (client)
        packet[0] = (3 << 6) | (4 << 3) | 3;
        match self.socket.send_to(packet, addr) {
            Ok(sent) => {
                if sent != SNTP_PACKET_SIZE {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid SNTP packet size sent",
                    ));
                }
                Ok(sent as u32)
            }
            Err(error) => Err(error),
        }
    }

    #[inline]
    fn recv_packet(&self, packet: &mut [u8]) -> SntpRawTimeResult {
        match self.socket.recv_from(packet) {
            Ok((recv, _)) => {
                if recv != SNTP_PACKET_SIZE {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid SNTP packet size received",
                    ));
                }
                let hdr = packet[0];
                let vn = (hdr & 0x38) >> 3;
                if vn != 4 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Server returned wrong SNTP version {vn}, expected 4.",
                    ));
                }
                let mode = hdr & 0x7;
                if mode != 4 && mode != 5 {
                    return Err(Error::new(ErrorKind::Other, "Not a SNTP server reply"));
                }
                self.kiss_of_death.set(packet[1] == 0);
                let timestamp = SntpTimestamp {
                    secs: read_be_u32(&packet[40..44]),
                    frac: read_be_u32(&packet[44..48]),
                };
                Ok(timestamp)
            }
            Err(error) => Err(error),
        }
    }

    /// If server returns `true`, the user should not send requests to it.
    pub fn is_kiss_of_death(&self) -> bool {
        self.kiss_of_death.get()
    }

    /// Sets the inactivity time to the client get time out. If not specified, the client assumes 5 seconds as default.
    pub fn set_timeout(&self, timeout: Duration) -> io::Result<()> {
        let dur = Some(timeout);
        self.socket.set_write_timeout(dur)?;
        self.socket.set_read_timeout(dur)
    }

    /// Obtains the raw time from a NTP server address.
    pub fn get_raw_time_by_addr<A: ToSocketAddrs>(&self, addr: A) -> SntpRawTimeResult {
        let mut packet = [0u8; SNTP_PACKET_SIZE];
        self.send_packet(addr, &mut packet)?;
        self.recv_packet(&mut packet)
    }

    /// Obtains the [Unix time](https://en.wikipedia.org/wiki/Unix_time) from a NTP server address.
    pub fn get_unix_time_by_addr<A: ToSocketAddrs>(&self, addr: A) -> SntpUnixTimeResult {
        let raw_time = self.get_raw_time_by_addr(addr)?;
        let raw_secs = raw_time.secs;
        Ok((raw_secs as i64) - (SNTP_TIME_OFFSET as i64))
    }

    /// Obtains the raw time from default NTP server address [`POOL_NTP_ADDR`](constant.POOL_NTP_ADDR.html).
    pub fn get_raw_time(&self) -> SntpRawTimeResult {
        self.get_raw_time_by_addr(POOL_NTP_ADDR)
    }

    /// Obtains the [Unix time](https://en.wikipedia.org/wiki/Unix_time) from default NTP server address [`POOL_NTP_ADDR`](constant.POOL_NTP_ADDR.html).
    pub fn get_unix_time(&self) -> SntpUnixTimeResult {
        self.get_unix_time_by_addr(POOL_NTP_ADDR)
    }
}
