/// How many products have fewer than `limit` left? ([12, 2, 6, 1], 5) -> 2
pub fn count_low(counts: &[u32], limit: u32) -> u32 {
    let mut low = 0;
    for n in counts {
        if *n < limit {
            low += 1;
        }
    }
    low
}

/// The biggest count on the shelf. An empty shelf gives 0.
pub fn largest(counts: &[u32]) -> u32 {
    let mut best = 0;
    for n in counts {
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
    fn count_low_finds_short_shelves() {
        assert_eq!(count_low(&[12, 2, 6, 1], 5), 2, "2 pens and 1 lamp are low");
        assert_eq!(count_low(&[12, 6], 5), 0);
        assert_eq!(count_low(&[], 5), 0);
    }

    #[test]
    fn largest_finds_the_biggest_count() {
        assert_eq!(largest(&[12, 40, 6]), 40);
        assert_eq!(largest(&[]), 0, "no products, nothing to compare");
    }
}
