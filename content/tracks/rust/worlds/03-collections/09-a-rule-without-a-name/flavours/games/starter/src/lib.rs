/// Every drop count plus the Looting `bonus`: ([1, 3], 2) -> [3, 5]
pub fn with_looting(drops: &[u32], bonus: u32) -> Vec<u32> {
    // TODO: write the looting rule as a closure that captures `bonus`
    let mut out = Vec::new();
    for n in drops {
        // TODO: push the drop count with the bonus, using your rule
        out.push(*n);
    }
    out
}

/// How much durability each tool is missing from `max`.
/// ([200, 31, 250], 250) -> [50, 219, 0]: a new tool needs nothing.
pub fn repairs_needed(durability: &[u32], max: u32) -> Vec<u32> {
    // TODO: write a closure that captures `max` and gives max minus the durability, never below 0 (saturating_sub)
    let mut out = Vec::new();
    for d in durability {
        // TODO: push the missing durability for this tool, using your rule
        out.push(*d);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_looting_adds_the_bonus_to_every_drop() {
        assert_eq!(with_looting(&[1, 3], 2), vec![3, 5]);
        assert_eq!(with_looting(&[4], 0), vec![4], "no Looting changes nothing");
        assert_eq!(with_looting(&[], 3), Vec::<u32>::new());
    }

    #[test]
    fn repairs_needed_bring_each_tool_to_max() {
        assert_eq!(repairs_needed(&[200, 31, 250], 250), vec![50, 219, 0]);
    }
}
