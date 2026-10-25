//! A slice of player counts is a pointer and a length.

/// Sums `len` player counts starting at `ptr`.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to `len` consecutive live i32s
/// that are not changed while this runs.
pub unsafe fn total_players_raw(ptr: *const i32, len: usize) -> i32 {
    // SAFETY: the caller guarantees ptr and len describe live i32s.
    let items = unsafe { std::slice::from_raw_parts(ptr, len) };
    items.iter().sum()
}

/// Sums a slice of player counts. Safe to call with any slice.
pub fn total_players(counts: &[i32]) -> i32 {
    // SAFETY: a slice's pointer and length always describe live items.
    unsafe { total_players_raw(counts.as_ptr(), counts.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_players_from_raw_parts() {
        let v = [10, 20, 30, 40];
        // SAFETY: pointer and lengths stay within `v`.
        unsafe {
            assert_eq!(total_players_raw(v.as_ptr(), 4), 100);
            assert_eq!(total_players_raw(v.as_ptr().add(2), 2), 70, "start at the third server");
            assert_eq!(total_players_raw(v.as_ptr(), 0), 0);
        }
    }

    #[test]
    fn safe_wrapper_totals_a_slice() {
        assert_eq!(total_players(&[1, 2, 3]), 6);
        assert_eq!(total_players(&[5, 6, 7][1..]), 13, "a slice from the middle");
        assert_eq!(total_players(&[]), 0, "an empty slice must work too");
    }
}
