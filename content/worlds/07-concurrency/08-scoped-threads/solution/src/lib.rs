use std::thread;

/// Sums the slice using two scoped threads (first half, second half).
pub fn parallel_sum(data: &[i64]) -> i64 {
    let (left, right) = data.split_at(data.len() / 2);
    thread::scope(|s| {
        let a = s.spawn(|| left.iter().sum::<i64>());
        let b = s.spawn(|| right.iter().sum::<i64>());
        a.join().unwrap() + b.join().unwrap()
    })
}

/// Largest value of `data`, found by one scoped thread per chunk of `chunk_size`.
/// Returns None for an empty slice. `chunk_size` is at least 1.
pub fn max_of_chunks(data: &[i32], chunk_size: usize) -> Option<i32> {
    thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(chunk_size)
            .map(|chunk| s.spawn(move || chunk.iter().copied().max()))
            .collect();
        handles.into_iter().filter_map(|h| h.join().unwrap()).max()
    })
}

/// Sets every element to `index * 2`. Two threads: one for each half of the slice.
pub fn fill_in_parallel(buf: &mut [u32]) {
    let mid = buf.len() / 2;
    let (left, right) = buf.split_at_mut(mid);
    thread::scope(|s| {
        s.spawn(|| {
            for (i, x) in left.iter_mut().enumerate() {
                *x = i as u32 * 2;
            }
        });
        s.spawn(|| {
            for (i, x) in right.iter_mut().enumerate() {
                *x = (mid + i) as u32 * 2;
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_sum_borrows_both_halves() {
        let data: Vec<i64> = (1..=1000).collect();
        assert_eq!(parallel_sum(&data), 500_500);
        assert_eq!(parallel_sum(&[]), 0);
        assert_eq!(parallel_sum(&[5]), 5);
        assert_eq!(parallel_sum(&data[..7]), 28);
    }

    #[test]
    fn max_of_chunks_joins_inside_the_scope() {
        assert_eq!(max_of_chunks(&[3, 9, 2, 7, 1, 8], 2), Some(9));
        assert_eq!(max_of_chunks(&[-4, -2, -9], 1), Some(-2));
        assert_eq!(max_of_chunks(&[4, 5, 6], 10), Some(6));
        assert_eq!(max_of_chunks(&[], 3), None);
    }

    #[test]
    fn fill_in_parallel_writes_disjoint_halves() {
        let mut buf = vec![0u32; 7];
        fill_in_parallel(&mut buf);
        assert_eq!(buf, vec![0, 2, 4, 6, 8, 10, 12]);

        let mut empty: Vec<u32> = Vec::new();
        fill_in_parallel(&mut empty);
        assert!(empty.is_empty());
    }
}
