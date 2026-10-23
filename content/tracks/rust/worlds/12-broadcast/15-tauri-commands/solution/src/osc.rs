//! OSC (Open Sound Control): how broadcast graphics and VJ software take cues, as small UDP packets.

#![allow(unused_imports)]

use std::net::{SocketAddr, UdpSocket};

use crate::*;

/// A string as OSC wants it: its bytes, a 0, then zeros up to a multiple of 4 bytes.
pub fn pad_str(out: &mut Vec<u8>, text: &str) {
    out.extend_from_slice(text.as_bytes());
    out.push(0);
    while !out.len().is_multiple_of(4) {
        out.push(0);
    }
}

/// Send one OSC message as one UDP packet.
pub fn send_osc(socket: &UdpSocket, to: SocketAddr, address: &str, args: &[OscArg]) -> std::io::Result<()> {
    socket.send_to(&encode(address, args), to).map(|_| ())
}

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
