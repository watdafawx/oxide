//! Ping: how long the network takes to answer.

use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

use crate::sensor::{Missing, Sensor};
use crate::stat::StatKind;

/// How long a TCP connection to `addr` takes, or None when it does not connect within `timeout`.
pub fn ping_once(addr: SocketAddr, timeout: Duration) -> Option<Duration> {
    let start = Instant::now();
    TcpStream::connect_timeout(&addr, timeout).ok()?;
    Some(start.elapsed())
}

/// Ping in milliseconds to a server you choose (Kinetik's Stats page has the address box).
pub struct PingSensor {
    addr: SocketAddr,
    timed_out: bool,
}

impl PingSensor {
    /// Cloudflare's DNS on the HTTPS port: answers almost everywhere.
    pub fn new() -> Self {
        PingSensor::to(SocketAddr::from(([1, 1, 1, 1], 443)))
    }

    pub fn to(addr: SocketAddr) -> Self {
        PingSensor { addr, timed_out: false }
    }
}

impl Default for PingSensor {
    fn default() -> Self {
        PingSensor::new()
    }
}

impl Sensor for PingSensor {
    fn kind(&self) -> StatKind {
        StatKind::Ping
    }

    fn read(&mut self) -> Option<f64> {
        let time = ping_once(self.addr, Duration::from_secs(1));
        self.timed_out = time.is_none();
        time.map(|t| t.as_secs_f64() * 1000.0)
    }

    fn missing(&self) -> Option<Missing> {
        if self.timed_out { Some(Missing::Timeout) } else { None }
    }
}
