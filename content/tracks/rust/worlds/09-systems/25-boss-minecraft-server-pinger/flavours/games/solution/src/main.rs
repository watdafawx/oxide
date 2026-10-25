use sys_boss_mc_pinger::*;
use std::io::Write;
use std::net::{SocketAddr, TcpListener};
use std::panic;
use std::thread;

/// A tiny fake Minecraft server: answers one status request with canned JSON.
fn fake_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    thread::spawn(move || {
        let Ok((mut stream, _)) = listener.accept() else { return };
        let _handshake = read_packet(&mut stream);
        let _request = read_packet(&mut stream);
        let json = r#"{"version":{"name":"1.21.1","protocol":767},"players":{"max":20,"online":3},"description":{"text":"Hello ","extra":[{"text":"from Oxide"}]}}"#;
        let mut body = Vec::new();
        write_varint(&mut body, 0);
        write_string(&mut body, json);
        let mut frame = Vec::new();
        write_varint(&mut frame, body.len() as i32);
        frame.extend_from_slice(&body);
        stream.write_all(&frame).ok();
    });
    addr
}

fn main() {
    println!("== Minecraft server pinger ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        // Pass an address (e.g. 192.168.50.3:25565) to ping a real server.
        let addr = match std::env::args().nth(1) {
            Some(text) => match text.parse() {
                Ok(addr) => addr,
                Err(_) => {
                    println!("'{text}' is not an address like 1.2.3.4:25565");
                    return;
                }
            },
            None => {
                println!("No address given: pinging a fake server on this machine.");
                fake_server()
            }
        };
        println!("Pinging {addr} ...");
        match ping(addr) {
            Ok(s) => println!(
                "{}\nVersion: {} (protocol {})\nPlayers: {}/{}",
                s.description_text, s.version_name, s.protocol, s.players_online, s.players_max
            ),
            Err(e) => println!("Ping failed: {e}"),
        }
    });
    if result.is_err() {
        println!("Finish `Packet::encode`, `status_request`, `handshake`, `read_packet`, `parse_status` and `ping` to ping a server.");
    }
}
