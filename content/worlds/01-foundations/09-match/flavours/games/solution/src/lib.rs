pub enum StackAction {
    Add { amount: u32 },
    Remove { amount: u32 },
    Rename(String),
    Inspect,
}

/// 0 enchantments -> "common", 1..=2 -> "uncommon", 3..=5 -> "rare", anything more -> "epic"
pub fn rarity(enchants: u32) -> &'static str {
    match enchants {
        0 => "common",
        1..=2 => "uncommon",
        3..=5 => "rare",
        _ => "epic",
    }
}

/// negative -> "corrupt", 0 -> "empty", 64 or more -> "full", anything else -> "partial"
pub fn stack_state(count: i32) -> &'static str {
    match count {
        n if n < 0 => "corrupt",
        0 => "empty",
        n if n >= 64 => "full",
        _ => "partial",
    }
}

/// Add and Remove change the count (never below 0), Rename and Inspect leave it alone.
pub fn apply(action: &StackAction, count: u32) -> u32 {
    match action {
        StackAction::Add { amount } => count + amount,
        StackAction::Remove { amount } => count.saturating_sub(*amount),
        StackAction::Rename(_) | StackAction::Inspect => count,
    }
}

/// The new name of a Rename action, or an empty String for anything else.
pub fn new_name(action: &StackAction) -> String {
    if let StackAction::Rename(name) = action {
        name.clone()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rarity_follows_enchant_ranges() {
        assert_eq!(rarity(0), "common");
        assert_eq!(rarity(2), "uncommon");
        assert_eq!(rarity(3), "rare");
        assert_eq!(rarity(9), "epic");
    }

    #[test]
    fn stack_state_guards_corrupt_empty_full_partial() {
        assert_eq!(stack_state(-4), "corrupt");
        assert_eq!(stack_state(0), "empty");
        assert_eq!(stack_state(64), "full");
        assert_eq!(stack_state(70), "full");
        assert_eq!(stack_state(8), "partial");
    }

    #[test]
    fn every_stack_action_is_handled() {
        assert_eq!(apply(&StackAction::Add { amount: 5 }, 10), 15);
        assert_eq!(apply(&StackAction::Remove { amount: 3 }, 10), 7);
        assert_eq!(apply(&StackAction::Remove { amount: 30 }, 10), 0, "never below 0");
        assert_eq!(apply(&StackAction::Inspect, 10), 10, "Inspect does not change the count");
        assert_eq!(apply(&StackAction::Rename("gem".to_string()), 10), 10);
    }

    #[test]
    fn new_name_is_pulled_out_with_if_let() {
        assert_eq!(new_name(&StackAction::Rename("Excalibur".to_string())), "Excalibur");
        assert_eq!(new_name(&StackAction::Inspect), "");
    }
}
