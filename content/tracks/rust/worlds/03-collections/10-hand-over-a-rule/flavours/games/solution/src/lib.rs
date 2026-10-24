/// How many stacks pass `rule`? count_where(&[64, 2, 64], |n| n == 64) -> 2
pub fn count_where(stacks: &[u32], rule: impl Fn(u32) -> bool) -> u32 {
    let mut matches = 0;
    for n in stacks {
        if rule(*n) {
            matches += 1;
        }
    }
    matches
}

/// Every drop count, changed by `change`. change_all(&[1, 3], |n| n * 2) -> [2, 6]
pub fn change_all(drops: &[u32], change: impl Fn(u32) -> u32) -> Vec<u32> {
    let mut out = Vec::new();
    for n in drops {
        out.push(change(*n));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_where_uses_the_callers_rule() {
        let stacks = [64, 2, 64, 0, 16];
        assert_eq!(count_where(&stacks, |n| n == 64), 2, "two full stacks");
        assert_eq!(count_where(&stacks, |n| n == 0), 1, "one empty slot");
        let limit = 10;
        assert_eq!(count_where(&stacks, |n| n > limit), 3, "a rule may capture");
    }

    #[test]
    fn change_all_applies_the_rule_to_every_drop() {
        assert_eq!(change_all(&[1, 3], |n| n * 2), vec![2, 6]);
        assert_eq!(change_all(&[1, 3], |n| n + 1), vec![2, 4]);
    }
}
