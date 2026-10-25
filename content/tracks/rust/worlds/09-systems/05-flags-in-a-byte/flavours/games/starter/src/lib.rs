//! Yes-or-no answers packed into the bits of one byte.

pub const WHITELIST: u8 = 0b0000_0001;
pub const PVP: u8 = 0b0000_0010;
// TODO: the third rule is the bit worth 4
pub const HARDCORE: u8 = 0;

/// Returns `rules` with the bits of `rule` turned on.
pub fn enable_rule(rules: u8, rule: u8) -> u8 {
    // TODO: turn the rule bits on, keep the others
    todo!()
}

/// True if every bit of `rule` is on in `rules`.
pub fn rule_enabled(rules: u8, rule: u8) -> bool {
    // TODO: are all the rule bits on in rules?
    todo!()
}

/// Returns `rules` with the bits of `rule` turned off.
pub fn disable_rule(rules: u8, rule: u8) -> u8 {
    // TODO: turn the rule bits off, keep the others
    todo!()
}

/// How many bits of `rules` are 1.
pub fn count_enabled_rules(rules: u32) -> u32 {
    // TODO: how many bits are on (integers have a method for it)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_turn_on_and_get_tested() {
        assert_eq!(HARDCORE, 0b100, "HARDCORE is the bit worth 4");
        let r = enable_rule(0, WHITELIST);
        assert_eq!(r, 0b001);
        let r = enable_rule(r, HARDCORE);
        assert_eq!(r, 0b101);
        assert_eq!(enable_rule(r, WHITELIST), 0b101, "enabling a rule that is on changes nothing");
        assert!(rule_enabled(r, WHITELIST));
        assert!(!rule_enabled(r, PVP));
        assert!(rule_enabled(r, WHITELIST | HARDCORE), "all bits of the rule must be on");
        assert!(!rule_enabled(r, WHITELIST | PVP), "one of the two rules is off");
    }

    #[test]
    fn disable_rule_leaves_the_rest() {
        assert_eq!(disable_rule(0b111, PVP), 0b101);
        assert_eq!(disable_rule(0b101, WHITELIST), 0b100);
        assert_eq!(disable_rule(0b100, WHITELIST), 0b100, "disabling a rule that is off changes nothing");
    }

    #[test]
    fn counts_enabled_rules() {
        assert_eq!(count_enabled_rules(0), 0);
        assert_eq!(count_enabled_rules(0b1011_0000), 3);
        assert_eq!(count_enabled_rules(u32::MAX), 32);
    }
}
