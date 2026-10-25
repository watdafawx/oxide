//! Minecraft Server List Ping client (protocol 767, Minecraft 1.21.1).

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use serde_json::Value;

/// Protocol version of Minecraft 1.21.1.
pub const PROTOCOL_VERSION: i32 = 767;
/// The protocol never sends frames longer than a 3-byte VarInt can describe.
pub const MAX_PACKET_LEN: i32 = 2_097_151;

fn invalid(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg.to_string())
}

// ---------------------------------------------------------------------------
// Already written for you (you built these in the VarInt quest).
// ---------------------------------------------------------------------------

/// Appends `value` as a VarInt.
pub fn write_varint(out: &mut Vec<u8>, value: i32) {
    let mut v = value as u32;
    loop {
        let mut byte = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if v == 0 {
            return;
        }
    }
}

/// Reads one VarInt (at most 5 bytes) from a stream.
pub fn read_varint(r: &mut impl Read) -> io::Result<i32> {
    let mut result: u32 = 0;
    for i in 0..5 {
        let mut b = [0u8; 1];
        r.read_exact(&mut b)?;
        result |= u32::from(b[0] & 0x7f) << (7 * i);
        if b[0] & 0x80 == 0 {
            return Ok(result as i32);
        }
    }
    Err(invalid("VarInt is longer than 5 bytes"))
}

/// Appends a protocol String: VarInt byte length, then UTF-8 bytes.
pub fn write_string(out: &mut Vec<u8>, s: &str) {
    write_varint(out, s.len() as i32);
    out.extend_from_slice(s.as_bytes());
}

/// Reads a protocol String from the start of `buf`.
pub fn read_string(buf: &[u8]) -> io::Result<String> {
    let mut cur = io::Cursor::new(buf);
    let len = read_varint(&mut cur)?;
    let start = cur.position() as usize;
    let len = usize::try_from(len).map_err(|_| invalid("negative string length"))?;
    let bytes = buf
        .get(start..start.saturating_add(len))
        .ok_or_else(|| invalid("string is longer than the packet"))?;
    String::from_utf8(bytes.to_vec()).map_err(|_| invalid("string is not UTF-8"))
}

// ---------------------------------------------------------------------------
// Your job.
// ---------------------------------------------------------------------------

/// One protocol packet: an id and its raw payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub id: i32,
    pub payload: Vec<u8>,
}

impl Packet {
    /// Frames the packet: VarInt length (of id + payload), VarInt id, payload.
    pub fn encode(&self) -> Vec<u8> {
        let mut body = Vec::new();
        write_varint(&mut body, self.id);
        body.extend_from_slice(&self.payload);
        let mut out = Vec::new();
        write_varint(&mut out, body.len() as i32);
        out.extend_from_slice(&body);
        out
    }
}

/// The Status Request packet (id 0, empty payload), ready to send.
pub fn status_request() -> Vec<u8> {
    Packet { id: 0, payload: Vec::new() }.encode()
}

/// The Handshake packet (id 0) asking for status (next state 1), ready to send.
/// Payload: VarInt protocol version, String host, u16 port big-endian, VarInt next state.
pub fn handshake(host: &str, port: u16) -> Vec<u8> {
    let mut payload = Vec::new();
    write_varint(&mut payload, PROTOCOL_VERSION);
    write_string(&mut payload, host);
    payload.extend_from_slice(&port.to_be_bytes());
    write_varint(&mut payload, 1);
    Packet { id: 0, payload }.encode()
}

/// Reads exactly one packet from `r`. The length must be 1..=MAX_PACKET_LEN.
pub fn read_packet(r: &mut impl Read) -> io::Result<Packet> {
    let len = read_varint(r)?;
    if !(1..=MAX_PACKET_LEN).contains(&len) {
        return Err(invalid("bad packet length"));
    }
    let mut body = vec![0u8; len as usize];
    r.read_exact(&mut body)?;
    let mut cur = io::Cursor::new(&body[..]);
    let id = read_varint(&mut cur)?;
    let start = cur.position() as usize;
    Ok(Packet { id, payload: body[start..].to_vec() })
}

/// What a server tells the server list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub version_name: String,
    pub protocol: i32,
    pub players_online: u32,
    pub players_max: u32,
    pub description_text: String,
}

/// Turns a `description` JSON value into plain text.
fn flatten_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Object(_) => {
            let mut out = v["text"].as_str().unwrap_or("").to_string();
            if let Some(extra) = v["extra"].as_array() {
                for part in extra {
                    out.push_str(&flatten_text(part));
                }
            }
            out
        }
        _ => String::new(),
    }
}

/// Parses the status JSON. Missing or malformed required fields are `InvalidData` errors.
pub fn parse_status(json: &str) -> io::Result<Status> {
    let v: Value = serde_json::from_str(json).map_err(|e| invalid(&e.to_string()))?;
    let version_name = v["version"]["name"]
        .as_str()
        .ok_or_else(|| invalid("missing version.name"))?
        .to_string();
    let protocol = v["version"]["protocol"]
        .as_i64()
        .ok_or_else(|| invalid("missing version.protocol"))? as i32;
    let players_online = v["players"]["online"]
        .as_u64()
        .ok_or_else(|| invalid("missing players.online"))? as u32;
    let players_max = v["players"]["max"]
        .as_u64()
        .ok_or_else(|| invalid("missing players.max"))? as u32;
    Ok(Status {
        version_name,
        protocol,
        players_online,
        players_max,
        description_text: flatten_text(&v["description"]),
    })
}

/// Pings the server at `addr`: handshake, status request, read and parse the response.
pub fn ping(addr: SocketAddr) -> io::Result<Status> {
    let timeout = Duration::from_secs(3);
    let mut stream = TcpStream::connect_timeout(&addr, timeout)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    stream.write_all(&handshake(&addr.ip().to_string(), addr.port()))?;
    stream.write_all(&status_request())?;
    let packet = read_packet(&mut stream)?;
    if packet.id != 0 {
        return Err(invalid("expected a Status Response (id 0)"));
    }
    let json = read_string(&packet.payload)?;
    parse_status(&json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Instant;

    fn varint(v: i32) -> Vec<u8> {
        let mut out = Vec::new();
        write_varint(&mut out, v);
        out
    }

    /// Independent framing for building test input.
    fn frame(id: i32, payload: &[u8]) -> Vec<u8> {
        let mut body = varint(id);
        body.extend_from_slice(payload);
        let mut out = varint(body.len() as i32);
        out.extend_from_slice(&body);
        out
    }

    #[test]
    fn packet_encode_frames_length_and_id() {
        let p = Packet { id: 0, payload: vec![0xaa, 0xbb] };
        assert_eq!(p.encode(), [3, 0, 0xaa, 0xbb]);

        // 200 bytes of payload + 1 byte of id = 201 = VarInt [0xc9, 0x01]
        let big = Packet { id: 0x1f, payload: vec![7; 200] };
        let mut expected = vec![0xc9, 0x01, 0x1f];
        expected.extend_from_slice(&[7; 200]);
        assert_eq!(big.encode(), expected, "length counts the id byte too");
    }

    #[test]
    fn status_request_is_two_bytes() {
        assert_eq!(status_request(), [0x01, 0x00]);
    }

    #[test]
    fn handshake_matches_protocol_bytes() {
        // length 16 | id 0 | 767 | "localhost" | 25565 | next state 1
        let mut expected = vec![16, 0x00, 0xff, 0x05, 9];
        expected.extend_from_slice(b"localhost");
        expected.extend_from_slice(&[0x63, 0xdd, 0x01]);
        assert_eq!(handshake("localhost", 25565), expected);
    }

    #[test]
    fn read_packet_parses_frames() {
        let mut bytes = frame(0, b"hello");
        bytes.extend_from_slice(&frame(1, &[9]));
        bytes.extend_from_slice(&frame(0x80, &[])); // two-byte id
        let mut r = Cursor::new(bytes);

        let a = read_packet(&mut r).expect("first packet");
        assert_eq!(a, Packet { id: 0, payload: b"hello".to_vec() });
        let b = read_packet(&mut r).expect("second packet: the first must not read too much");
        assert_eq!(b, Packet { id: 1, payload: vec![9] });
        let c = read_packet(&mut r).expect("third packet");
        assert_eq!(c, Packet { id: 0x80, payload: vec![] });
    }

    #[test]
    fn read_packet_rejects_bad_frames() {
        let e = read_packet(&mut Cursor::new(vec![5, 0, 1])).unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::UnexpectedEof, "5 bytes promised, 2 delivered");

        assert!(read_packet(&mut Cursor::new(Vec::new())).is_err(), "empty input");

        let zero = read_packet(&mut Cursor::new(vec![0])).unwrap_err();
        assert_eq!(zero.kind(), io::ErrorKind::InvalidData, "length 0 has no room for an id");

        let negative = read_packet(&mut Cursor::new(varint(-1))).unwrap_err();
        assert_eq!(negative.kind(), io::ErrorKind::InvalidData);

        let huge = read_packet(&mut Cursor::new(vec![0xff, 0xff, 0xff, 0x7f])).unwrap_err();
        assert_eq!(huge.kind(), io::ErrorKind::InvalidData, "268 million bytes is not a packet");
    }

    #[test]
    fn parse_status_reads_fields() {
        let json = r#"{"version":{"name":"1.21.1","protocol":767},
            "players":{"max":20,"online":3,"sample":[]},
            "description":"A Survival server"}"#;
        assert_eq!(
            parse_status(json).expect("valid status"),
            Status {
                version_name: "1.21.1".into(),
                protocol: 767,
                players_online: 3,
                players_max: 20,
                description_text: "A Survival server".into(),
            }
        );
        assert!(parse_status("not json").is_err());
        assert!(parse_status("{}").is_err(), "required fields are missing");
    }

    #[test]
    fn parse_status_flattens_description_components() {
        let json = r#"{"version":{"name":"Paper 1.21.1","protocol":767},
            "players":{"max":100,"online":0},
            "description":{"text":"Hello ","extra":[{"text":"Survival","bold":true},"!"]}}"#;
        assert_eq!(parse_status(json).unwrap().description_text, "Hello Survival!");

        let plain = r#"{"version":{"name":"x","protocol":1},"players":{"max":1,"online":1},
            "description":{"text":"just text"}}"#;
        assert_eq!(parse_status(plain).unwrap().description_text, "just text");
    }

    // ---- fake server -------------------------------------------------------

    fn accept_within(listener: &TcpListener, wait: Duration) -> io::Result<TcpStream> {
        listener.set_nonblocking(true)?;
        let deadline = Instant::now() + wait;
        loop {
            match listener.accept() {
                Ok((s, _)) => {
                    s.set_nonblocking(false)?;
                    return Ok(s);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock && Instant::now() < deadline => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Reads one frame and returns its body (id + payload).
    fn read_frame(s: &mut TcpStream) -> io::Result<Vec<u8>> {
        let len = read_varint(s)?;
        if !(1..=4096).contains(&len) {
            return Err(invalid("fake server: bad frame length"));
        }
        let mut body = vec![0u8; len as usize];
        s.read_exact(&mut body)?;
        Ok(body)
    }

    /// Accepts one client, records its two packets, answers with a status response.
    fn fake_server(listener: TcpListener) -> thread::JoinHandle<io::Result<(Vec<u8>, Vec<u8>)>> {
        thread::spawn(move || {
            let mut s = accept_within(&listener, Duration::from_secs(2))?;
            s.set_read_timeout(Some(Duration::from_secs(2)))?;
            let handshake_body = read_frame(&mut s)?;
            let request_body = read_frame(&mut s)?;

            let json = r#"{"version":{"name":"1.21.1","protocol":767},"players":{"max":20,"online":3,"sample":[{"name":"Notch","id":"069a79f4-44e9-4726-a5be-fca90e38aaf5"}]},"description":{"text":"A test server"},"enforcesSecureChat":true}"#;
            let mut body = vec![0x00];
            write_string(&mut body, json);
            let mut out = varint(body.len() as i32);
            out.extend_from_slice(&body);
            s.write_all(&out)?;
            Ok((handshake_body, request_body))
        })
    }

    #[test]
    fn ping_talks_to_fake_server() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = fake_server(listener);

        let result = ping(addr);
        let (handshake_body, request_body) = server
            .join()
            .unwrap()
            .expect("the client should send a handshake and a status request");

        let mut expected = vec![0x00, 0xff, 0x05, 9];
        expected.extend_from_slice(b"127.0.0.1");
        expected.extend_from_slice(&addr.port().to_be_bytes());
        expected.push(1);
        assert_eq!(handshake_body, expected, "handshake packet as the server saw it");
        assert_eq!(request_body, [0x00], "status request packet as the server saw it");

        assert_eq!(
            result.expect("ping failed"),
            Status {
                version_name: "1.21.1".into(),
                protocol: 767,
                players_online: 3,
                players_max: 20,
                description_text: "A test server".into(),
            }
        );
    }
}
