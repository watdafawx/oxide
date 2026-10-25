//! Four bytes of an IPv4 address in one u32.

/// Packs `a.b.c.d` into one u32: `a` in the top byte, `d` in the lowest.
/// `192.168.50.3` becomes `0xC0A8_3203`.
pub fn pack_ipv4(a: u8, b: u8, c: u8, d: u8) -> u32 {
    // TODO: widen each part, shift it to its place, and join them
    todo!()
}

/// Splits a packed address back into `(a, b, c, d)`.
pub fn unpack_ipv4(ip: u32) -> (u8, u8, u8, u8) {
    // TODO: shift each byte down to the bottom and keep only those 8 bits
    todo!()
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
