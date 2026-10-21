/// The chest held 10 logs; you pick up 5 more.
pub fn logs_after_pickup() -> u32 {
    let logs = 10;
    // TODO: add 5 to `logs` here. When Rust complains, add `mut` to the line above.
    logs
}

/// You had 48 planks and crafted a crafting table, which costs 4.
pub fn planks_after_crafting() -> u32 {
    let planks = 48;
    // TODO: take 4 away from `planks`, same shape as above
    planks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picking_up_five_logs() {
        assert_eq!(logs_after_pickup(), 15, "10 logs + 5 picked up");
    }

    #[test]
    fn crafting_uses_four_planks() {
        assert_eq!(planks_after_crafting(), 44, "48 planks - 4 for the table");
    }
}
