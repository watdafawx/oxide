/// How many logs are in the chest right now.
pub fn logs_in_chest() -> u32 {
    let logs = 0; // TODO: the chest holds 12 logs
    logs
}

/// How many planks those logs make. One log makes four planks.
pub fn planks_in_chest() -> u32 {
    let logs = 12;
    let planks = 0; // TODO: four planks per log, so logs * 4
    planks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chest_holds_twelve_logs() {
        assert_eq!(logs_in_chest(), 12, "the chest holds 12 logs");
    }

    #[test]
    fn twelve_logs_make_48_planks() {
        assert_eq!(planks_in_chest(), 48, "12 logs, four planks each");
    }
}
