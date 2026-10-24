//! A MOTD colour: three bytes of red, green and blue in one u32.

/// Packs a colour into 0x00RRGGBB: red in bits 16-23, green in 8-15, blue in 0-7.
pub fn pack_color(r: u8, g: u8, b: u8) -> u32 {
    // TODO: widen each part, shift it to its place, and join them
    todo!()
}

/// Splits 0x00RRGGBB back into (r, g, b).
pub fn unpack_color(color: u32) -> (u8, u8, u8) {
    // TODO: shift each byte down to the bottom and keep only those 8 bits
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motd_color_packs_into_u32() {
        assert_eq!(pack_color(0xff, 0xaa, 0x00), 0x00ff_aa00, "gold, the MOTD colour");
        assert_eq!(pack_color(0x55, 0xff, 0x55), 0x0055_ff55);
        assert_eq!(pack_color(0, 0, 0), 0);
    }

    #[test]
    fn motd_color_unpacks_from_u32() {
        assert_eq!(unpack_color(0x00ff_aa00), (0xff, 0xaa, 0x00));
        assert_eq!(unpack_color(0x0055_ff55), (0x55, 0xff, 0x55));
        assert_eq!(unpack_color(pack_color(1, 2, 3)), (1, 2, 3));
    }
}
