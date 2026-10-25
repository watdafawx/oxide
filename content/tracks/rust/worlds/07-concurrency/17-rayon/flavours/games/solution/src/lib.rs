use rayon::prelude::*;

/// A stand-in hash for every jar size (size * 31), in the same order, in parallel.
pub fn checksums(sizes: &[u64]) -> Vec<u64> {
    sizes.par_iter().map(|size| size * 31).collect()
}

/// All jar sizes added up: the modpack download size, in parallel.
pub fn total_size(sizes: &[u64]) -> u64 {
    sizes.par_iter().sum()
}

/// How many jars are bigger than `limit`, in parallel. Takes the Vec.
pub fn count_big(sizes: Vec<u64>, limit: u64) -> usize {
    sizes.into_par_iter().filter(|size| *size > limit).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksums_keep_the_input_order() {
        let sizes: Vec<u64> = (1..=1000).collect();
        let sums = checksums(&sizes);
        assert_eq!(sums.len(), 1000);
        assert_eq!(&sums[..3], &[31, 62, 93]);
        assert_eq!(sums[999], 31_000);
    }

    #[test]
    fn total_size_adds_up_in_parallel() {
        let sizes: Vec<u64> = (1..=1000).collect();
        assert_eq!(total_size(&sizes), 500_500);
        assert_eq!(total_size(&[]), 0);
    }

    #[test]
    fn count_big_takes_the_list() {
        assert_eq!(count_big(vec![10, 5_000, 300, 9_000, 4_096], 4_096), 2);
        assert_eq!(count_big(Vec::new(), 0), 0);
    }
}
