#[derive(Debug, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

/// 0 enchantments -> Common, 1..=2 -> Uncommon, 3..=5 -> Rare, anything more -> Epic
pub fn rarity(enchants: u32) -> Rarity {
    match enchants {
        0 => Rarity::Common,
        1..=2 => Rarity::Uncommon,
        3..=5 => Rarity::Rare,
        _ => Rarity::Epic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn few_enchants_are_common_or_uncommon() {
        assert_eq!(rarity(0), Rarity::Common);
        assert_eq!(rarity(1), Rarity::Uncommon);
        assert_eq!(rarity(2), Rarity::Uncommon);
    }

    #[test]
    fn many_enchants_are_rare_or_epic() {
        assert_eq!(rarity(3), Rarity::Rare);
        assert_eq!(rarity(5), Rarity::Rare);
        assert_eq!(rarity(6), Rarity::Epic);
        assert_eq!(rarity(99), Rarity::Epic);
    }
}
