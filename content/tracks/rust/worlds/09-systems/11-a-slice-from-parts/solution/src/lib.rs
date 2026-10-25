//! A slice is a pointer and a length.

/// Sums `len` numbers starting at `ptr`.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to `len` consecutive live i32s
/// that are not changed while this runs.
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    // SAFETY: the caller guarantees ptr and len describe live i32s.
    let items = unsafe { std::slice::from_raw_parts(ptr, len) };
    items.iter().sum()
}

/// Sums a slice of numbers. Safe to call with any slice.
pub fn safe_sum(slice: &[i32]) -> i32 {
    // SAFETY: a slice's pointer and length always describe live items.
    unsafe { sum_raw(slice.as_ptr(), slice.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_raw_parts() {
        let v = [10, 20, 30, 40];
        // SAFETY: pointer and lengths stay within `v`.
        unsafe {
            assert_eq!(sum_raw(v.as_ptr(), 4), 100);
            assert_eq!(sum_raw(v.as_ptr().add(2), 2), 70, "start at the third item");
            assert_eq!(sum_raw(v.as_ptr(), 0), 0);
        }
    }

    #[test]
    fn safe_wrapper_sums_slice() {
        assert_eq!(safe_sum(&[1, 2, 3]), 6);
        assert_eq!(safe_sum(&[5, 6, 7][1..]), 13, "a slice from the middle");
        assert_eq!(safe_sum(&[]), 0, "an empty slice must work too");
    }
}
