#[derive(Debug, PartialEq)]
pub enum Tier {
    Basic,
    Standard,
    Premium,
    Deluxe,
}

/// 0 extras -> Basic, 1..=2 -> Standard, 3..=5 -> Premium, anything more -> Deluxe
pub fn tier(extras: u32) -> Tier {
    // TODO: match extras { 0 => ..., 1..=2 => ..., 3..=5 => ..., _ => ... }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn few_extras_are_basic_or_standard() {
        assert_eq!(tier(0), Tier::Basic);
        assert_eq!(tier(1), Tier::Standard);
        assert_eq!(tier(2), Tier::Standard);
    }

    #[test]
    fn many_extras_are_premium_or_deluxe() {
        assert_eq!(tier(3), Tier::Premium);
        assert_eq!(tier(5), Tier::Premium);
        assert_eq!(tier(6), Tier::Deluxe);
        assert_eq!(tier(99), Tier::Deluxe);
    }
}
