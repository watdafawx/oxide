//! Yes-or-no answers packed into the bits of one byte.

pub const ONLINE: u8 = 0b0000_0001;
pub const FULL: u8 = 0b0000_0010;
// TODO: the third flag is the bit worth 4
pub const MAINTENANCE: u8 = 0;

/// Returns `value` with the bits of `flag` turned on.
pub fn set_flag(value: u8, flag: u8) -> u8 {
    // TODO: turn the flag bits on, keep the others
    todo!()
}

/// True if every bit of `flag` is on in `value`.
pub fn has_flag(value: u8, flag: u8) -> bool {
    // TODO: are all the flag bits on in value?
    todo!()
}

/// Returns `value` with the bits of `flag` turned off.
pub fn clear_flag(value: u8, flag: u8) -> u8 {
    // TODO: turn the flag bits off, keep the others
    todo!()
}

/// How many bits of `value` are 1.
pub fn count_set_bits(value: u32) -> u32 {
    // TODO: how many bits are on (integers have a method for it)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_set_and_test() {
        assert_eq!(MAINTENANCE, 0b100, "MAINTENANCE is the bit worth 4");
        let s = set_flag(0, ONLINE);
        assert_eq!(s, 0b001);
        let s = set_flag(s, MAINTENANCE);
        assert_eq!(s, 0b101);
        assert_eq!(set_flag(s, ONLINE), 0b101, "setting a bit that is on changes nothing");
        assert!(has_flag(s, ONLINE));
        assert!(!has_flag(s, FULL));
        assert!(has_flag(s, ONLINE | MAINTENANCE), "all bits of the mask must be on");
        assert!(!has_flag(s, ONLINE | FULL), "one of the two bits is off");
    }

    #[test]
    fn clear_flag_leaves_the_rest() {
        assert_eq!(clear_flag(0b111, FULL), 0b101);
        assert_eq!(clear_flag(0b101, ONLINE), 0b100);
        assert_eq!(clear_flag(0b100, ONLINE), 0b100, "clearing a bit that is off changes nothing");
    }

    #[test]
    fn counts_set_bits() {
        assert_eq!(count_set_bits(0), 0);
        assert_eq!(count_set_bits(0b1011_0000), 3);
        assert_eq!(count_set_bits(u32::MAX), 32);
    }
}
