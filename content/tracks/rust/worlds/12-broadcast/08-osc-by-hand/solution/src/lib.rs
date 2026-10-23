//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;
pub mod feed;
pub mod dedup;
pub mod grace;
pub mod moments;
pub mod triggers;
pub mod osc;

#[allow(unused_imports)]
pub use frames::*;
#[allow(unused_imports)]
pub use event::*;
#[allow(unused_imports)]
pub use feed::*;
#[allow(unused_imports)]
pub use dedup::*;
#[allow(unused_imports)]
pub use grace::*;
#[allow(unused_imports)]
pub use moments::*;
#[allow(unused_imports)]
pub use triggers::*;
#[allow(unused_imports)]
pub use osc::*;

#[allow(unused_imports)]
use std::net::{SocketAddr, UdpSocket};
#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use axum::extract::ws::{Message, WebSocket};
#[allow(unused_imports)]
use image::{RgbaImage, imageops};
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use tauri::{AppHandle, Emitter, Runtime};
#[allow(unused_imports)]
use tokio::io::AsyncReadExt;
#[allow(unused_imports)]
use tokio::net::TcpStream;
#[allow(unused_imports)]
use tokio::sync::{broadcast, mpsc};

/// One OSC message: the address, the type tags (a comma, then one letter per argument), then the arguments.
/// Numbers are 4 bytes, big-endian.
pub fn encode(address: &str, args: &[OscArg]) -> Vec<u8> {
    let mut out = Vec::new();
    pad_str(&mut out, address);
    let tags: String = std::iter::once(',')
        .chain(args.iter().map(|arg| match arg {
            OscArg::Int(_) => 'i',
            OscArg::Float(_) => 'f',
            OscArg::Str(_) => 's',
        }))
        .collect();
    pad_str(&mut out, &tags);
    for arg in args {
        match arg {
            OscArg::Int(v) => out.extend_from_slice(&v.to_be_bytes()),
            OscArg::Float(v) => out.extend_from_slice(&v.to_be_bytes()),
            OscArg::Str(s) => pad_str(&mut out, s),
        }
    }
    out
}

#[cfg(test)]
#[allow(unused_imports, dead_code, unused_variables, unused_mut)]
mod tests {
    use super::*;
    use std::time::Duration;

    const MATCH: &str = include_str!("../fixtures/match.jsonl");

    /// The recorded match's frames, without their times.
    fn frames() -> Vec<serde_json::Value> {
        MATCH.lines().map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap()["frame"].clone()).collect()
    }

    /// The recorded match's frames, back to back, as the Stats API sends them.
    fn wire() -> Vec<u8> {
        frames().iter().flat_map(|f| serde_json::to_vec(f).unwrap()).collect()
    }

    /// Every event in the recorded match, with its time.
    fn events() -> Vec<(Duration, GameEvent)> {
        MATCH
            .lines()
            .filter_map(|line| {
                let v: serde_json::Value = serde_json::from_str(line).unwrap();
                normalize(v["frame"].clone()).map(|e| (Duration::from_millis(v["t_ms"].as_u64().unwrap()), e))
            })
            .collect()
    }

    fn goal(scorer: &str, team_num: u8, goal_time: u32) -> Goal {
        Goal { scorer: PlayerRef { name: scorer.into(), team_num }, assister: None, goal_time }
    }

    fn secs(s: f64) -> Duration {
        Duration::from_secs_f64(s)
    }

    fn triggers() -> Vec<Trigger> {
        load_triggers(include_str!("../fixtures/triggers.json")).unwrap()
    }

    #[test]
    fn osc_bytes_match_spec() {
        let mut want = b"/oscillator/4/frequency\0,f\0\0".to_vec();
        want.extend_from_slice(&[0x43, 0xdc, 0x00, 0x00]);
        assert_eq!(encode("/oscillator/4/frequency", &[OscArg::Float(440.0)]), want);
        let mixed = encode("/hud", &[OscArg::Int(1), OscArg::Str("Wren".into())]);
        assert_eq!(mixed, b"/hud\0\0\0\0,is\0\0\0\0\x01Wren\0\0\0\0".to_vec());
    }

    #[test]
    fn udp_packet_arrives() {
        let notch = UdpSocket::bind("127.0.0.1:0").unwrap();
        notch.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
        let hud = UdpSocket::bind("127.0.0.1:0").unwrap();
        send_osc(&hud, notch.local_addr().unwrap(), "/notch/30s", &[OscArg::Int(1)]).unwrap();
        let mut buf = [0u8; 64];
        let (n, _) = notch.recv_from(&mut buf).unwrap();
        assert_eq!(&buf[..n], encode("/notch/30s", &[OscArg::Int(1)]).as_slice());
    }
}
