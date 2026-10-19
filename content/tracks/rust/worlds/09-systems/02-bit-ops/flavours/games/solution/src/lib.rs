//! Bit operations for server rules and MOTD colors.

pub const WHITELIST: u8 = 1 << 0;
pub const PVP: u8 = 1 << 1;
pub const HARDCORE: u8 = 1 << 2;

/// Returns `rules` with the bits of `rule` turned on.
pub fn enable_rule(rules: u8, rule: u8) -> u8 {
    rules | rule
}

/// Returns `rules` with the bits of `rule` turned off.
pub fn disable_rule(rules: u8, rule: u8) -> u8 {
    rules & !rule
}

/// True if every bit of `rule` is set in `rules`.
pub fn rule_enabled(rules: u8, rule: u8) -> bool {
    rules & rule == rule
}

/// Packs three color bytes into 0x00RRGGBB.
pub fn pack_color(r: u8, g: u8, b: u8) -> u32 {
    (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)
}

/// Splits 0x00RRGGBB back into (r, g, b).
pub fn unpack_color(color: u32) -> (u8, u8, u8) {
    ((color >> 16) as u8, (color >> 8) as u8, color as u8)
}

/// How many bits of `rules` are 1.
pub fn count_enabled_rules(rules: u32) -> u32 {
    rules.count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_turn_on_off_and_get_tested() {
        let r = enable_rule(0, WHITELIST);
        assert_eq!(r, 0b001);
        let r = enable_rule(r, HARDCORE);
        assert_eq!(r, 0b101);
        assert!(rule_enabled(r, WHITELIST));
        assert!(!rule_enabled(r, PVP));
        assert!(rule_enabled(r, WHITELIST | HARDCORE), "all bits of the rule must be set");
        assert!(!rule_enabled(r, WHITELIST | PVP));
        assert_eq!(disable_rule(r, WHITELIST), 0b100);
        assert_eq!(disable_rule(0b100, WHITELIST), 0b100, "clearing an unset bit changes nothing");
    }

    #[test]
    fn motd_color_packs_into_u32() {
        assert_eq!(pack_color(0xff, 0x80, 0x07), 0x00ff_8007);
        assert_eq!(unpack_color(0x00ff_8007), (0xff, 0x80, 0x07));
        assert_eq!(unpack_color(pack_color(1, 2, 3)), (1, 2, 3));
    }

    #[test]
    fn counts_enabled_rules() {
        assert_eq!(count_enabled_rules(0), 0);
        assert_eq!(count_enabled_rules(0b1011_0000), 3);
        assert_eq!(count_enabled_rules(u32::MAX), 32);
    }
}
