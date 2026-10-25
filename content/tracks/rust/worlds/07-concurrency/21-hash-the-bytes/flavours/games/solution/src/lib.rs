/// FNV-1a, 64 bit: the start value ("offset basis").
pub const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
/// FNV-1a, 64 bit: the prime to multiply by after each byte.
pub const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// FNV-1a hash of `data`: start at FNV_OFFSET; for each byte,
/// XOR the byte in, then wrapping-multiply by FNV_PRIME.
pub fn fnv1a(data: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET;
    for byte in data {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a_of_nothing_is_the_start_value() {
        assert_eq!(fnv1a(b""), 0xcbf29ce484222325);
    }

    #[test]
    fn fnv1a_matches_the_published_values() {
        assert_eq!(fnv1a(b"a"), 0xaf63dc4c8601ec8c);
        assert_eq!(fnv1a(b"foobar"), 0x85944171f73967e8);
        assert_ne!(fnv1a(b"sodium-0.5.8.jar"), fnv1a(b"sodium-0.5.9.jar"), "one changed byte, a new hash");
    }
}
