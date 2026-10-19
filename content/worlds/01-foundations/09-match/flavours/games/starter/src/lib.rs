pub enum StackAction {
    Add { amount: u32 },
    Remove { amount: u32 },
    Rename(String),
    Inspect,
}

/// 0 enchantments -> "common", 1..=2 -> "uncommon", 3..=5 -> "rare", anything more -> "epic"
pub fn rarity(enchants: u32) -> &'static str {
    // TODO: match on ranges; the catch-all arm is `_`
    todo!()
}

/// negative -> "corrupt", 0 -> "empty", 64 or more -> "full", anything else -> "partial"
pub fn stack_state(count: i32) -> &'static str {
    // TODO: match with guards, e.g. `n if n < 0 => ...` (first match wins)
    todo!()
}

/// Add and Remove change the count (never below 0), Rename and Inspect leave it alone.
pub fn apply(action: &StackAction, count: u32) -> u32 {
    // TODO: match on every StackAction variant
    todo!()
}

/// The new name of a Rename action, or an empty String for anything else.
pub fn new_name(action: &StackAction) -> String {
    // TODO: use `if let StackAction::Rename(name) = action { ... } else { ... }`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grades_use_ranges() {
        assert_eq!(rarity(0), "common");
        assert_eq!(rarity(2), "uncommon");
        assert_eq!(rarity(3), "rare");
        assert_eq!(rarity(9), "epic");
    }

    #[test]
    fn describe_number_uses_guards() {
        assert_eq!(stack_state(-4), "corrupt");
        assert_eq!(stack_state(0), "empty");
        assert_eq!(stack_state(64), "full");
        assert_eq!(stack_state(70), "full");
        assert_eq!(stack_state(8), "partial");
    }

    #[test]
    fn apply_moves_the_position() {
        assert_eq!(apply(&StackAction::Add { amount: 5 }, 10), 15);
        assert_eq!(apply(&StackAction::Remove { amount: 3 }, 10), 7);
        assert_eq!(apply(&StackAction::Remove { amount: 30 }, 10), 0, "never below 0");
        assert_eq!(apply(&StackAction::Inspect, 10), 10, "Inspect does not change the count");
        assert_eq!(apply(&StackAction::Rename("gem".to_string()), 10), 10);
    }

    #[test]
    fn say_text_uses_if_let() {
        assert_eq!(new_name(&StackAction::Rename("Excalibur".to_string())), "Excalibur");
        assert_eq!(new_name(&StackAction::Inspect), "");
    }
}
