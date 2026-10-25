/// How many tools have less than `limit` durability left? ([250, 3, 60, 1], 10) -> 2
pub fn count_breaking(durability: &[u32], limit: u32) -> u32 {
    let mut worn = 0;
    for d in durability {
        if *d < limit {
            worn += 1;
        }
    }
    worn
}

/// The biggest stack in the inventory. An empty inventory gives 0.
pub fn biggest_stack(stacks: &[u32]) -> u32 {
    let mut best = 0;
    for n in stacks {
        if *n > best {
            best = *n;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_breaking_finds_worn_tools() {
        assert_eq!(count_breaking(&[250, 3, 60, 1], 10), 2, "the 3 and the 1 are about to break");
        assert_eq!(count_breaking(&[250, 60], 10), 0);
        assert_eq!(count_breaking(&[], 10), 0);
    }

    #[test]
    fn biggest_stack_finds_the_fullest_slot() {
        assert_eq!(biggest_stack(&[16, 64, 8]), 64);
        assert_eq!(biggest_stack(&[]), 0, "empty inventory, nothing to compare");
    }
}
