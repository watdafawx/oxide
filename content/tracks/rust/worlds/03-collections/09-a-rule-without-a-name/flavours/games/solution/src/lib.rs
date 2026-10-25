/// Every drop count plus the Looting `bonus`: ([1, 3], 2) -> [3, 5]
pub fn with_looting(drops: &[u32], bonus: u32) -> Vec<u32> {
    let looted = |n: u32| n + bonus;
    let mut out = Vec::new();
    for n in drops {
        out.push(looted(*n));
    }
    out
}

/// How much durability each tool is missing from `max`.
/// ([200, 31, 250], 250) -> [50, 219, 0]: a new tool needs nothing.
pub fn repairs_needed(durability: &[u32], max: u32) -> Vec<u32> {
    let missing = |d: u32| max.saturating_sub(d);
    let mut out = Vec::new();
    for d in durability {
        out.push(missing(*d));
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
