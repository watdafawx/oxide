//! Bit operations.

pub const READ: u8 = 1 << 0;
pub const WRITE: u8 = 1 << 1;
pub const EXEC: u8 = 1 << 2;

/// Returns `value` with the bits of `flag` turned on.
pub fn set_flag(value: u8, flag: u8) -> u8 {
    // TODO: OR them together
    todo!()
}

/// Returns `value` with the bits of `flag` turned off.
pub fn clear_flag(value: u8, flag: u8) -> u8 {
    // TODO: AND with the NOT of flag
    todo!()
}

/// True if every bit of `flag` is set in `value`.
pub fn has_flag(value: u8, flag: u8) -> bool {
    // TODO: value & flag, compared with flag
    todo!()
}

/// Packs three color bytes into 0x00RRGGBB.
pub fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    // TODO: shift r left 16, g left 8, OR with b (convert to u32 first)
    todo!()
}

/// Splits 0x00RRGGBB back into (r, g, b).
pub fn unpack_rgb(color: u32) -> (u8, u8, u8) {
    // TODO: shift right by 16 / 8 / 0, then `as u8` keeps the low 8 bits
    todo!()
}

/// How many bits of `value` are 1.
pub fn count_set_bits(value: u32) -> u32 {
    // TODO: count the 1 bits (integers have a built-in method for this)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_set_clear_and_test() {
        let p = set_flag(0, READ);
        assert_eq!(p, 0b001);
        let p = set_flag(p, EXEC);
        assert_eq!(p, 0b101);
        assert!(has_flag(p, READ));
        assert!(!has_flag(p, WRITE));
        assert!(has_flag(p, READ | EXEC), "all bits of the flag must be set");
        assert!(!has_flag(p, READ | WRITE));
        assert_eq!(clear_flag(p, READ), 0b100);
        assert_eq!(clear_flag(0b100, READ), 0b100, "clearing an unset bit changes nothing");
    }

    #[test]
    fn rgb_packs_into_u32() {
        assert_eq!(pack_rgb(0xff, 0x80, 0x07), 0x00ff_8007);
        assert_eq!(unpack_rgb(0x00ff_8007), (0xff, 0x80, 0x07));
        assert_eq!(unpack_rgb(pack_rgb(1, 2, 3)), (1, 2, 3));
    }

    #[test]
    fn counts_set_bits() {
        assert_eq!(count_set_bits(0), 0);
        assert_eq!(count_set_bits(0b1011_0000), 3);
        assert_eq!(count_set_bits(u32::MAX), 32);
    }
}
