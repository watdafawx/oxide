//! A packet dump: a server's bytes written the way the protocol wiki writes them.

/// The packet bytes as two-digit lowercase hex, separated by single spaces.
/// `[0x10, 0x00, 0xff]` becomes `"10 00 ff"`.
pub fn packet_dump(packet: &[u8]) -> String {
    packet
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handshake_start_dumps_as_hex_pairs() {
        assert_eq!(packet_dump(&[0x10, 0x00, 0xff, 0x05]), "10 00 ff 05");
        assert_eq!(packet_dump(&[255, 16]), "ff 10", "255 is ff, 16 is 10");
    }

    #[test]
    fn small_bytes_keep_their_zero() {
        assert_eq!(packet_dump(&[1, 10]), "01 0a", "every byte is exactly two digits");
        assert_eq!(packet_dump(&[]), "", "no bytes, no text");
    }
}
