//! Four bytes of an IPv4 address in one u32.

/// Packs `a.b.c.d` into one u32: `a` in the top byte, `d` in the lowest.
/// `192.168.50.3` becomes `0xC0A8_3203`.
pub fn pack_ipv4(a: u8, b: u8, c: u8, d: u8) -> u32 {
    (u32::from(a) << 24) | (u32::from(b) << 16) | (u32::from(c) << 8) | u32::from(d)
}

/// Splits a packed address back into `(a, b, c, d)`.
pub fn unpack_ipv4(ip: u32) -> (u8, u8, u8, u8) {
    ((ip >> 24) as u8, (ip >> 16) as u8, (ip >> 8) as u8, ip as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_an_address_into_u32() {
        assert_eq!(pack_ipv4(192, 168, 50, 3), 0xC0A8_3203);
        assert_eq!(pack_ipv4(127, 0, 0, 1), 0x7F00_0001);
        assert_eq!(pack_ipv4(0, 0, 0, 0), 0);
    }

    #[test]
    fn unpacks_u32_into_an_address() {
        assert_eq!(unpack_ipv4(0xC0A8_3203), (192, 168, 50, 3));
        assert_eq!(unpack_ipv4(0x7F00_0001), (127, 0, 0, 1));
        assert_eq!(unpack_ipv4(pack_ipv4(10, 20, 30, 40)), (10, 20, 30, 40));
    }
}
