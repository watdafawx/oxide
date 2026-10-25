//! A slice is a pointer and a length.

/// Sums `len` numbers starting at `ptr`.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to `len` consecutive live i32s
/// that are not changed while this runs.
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    // TODO: turn the pointer and length back into a slice (unsafe, with a
    //       // SAFETY: comment), then add the items up
    todo!()
}

/// Sums a slice of numbers. Safe to call with any slice.
pub fn safe_sum(slice: &[i32]) -> i32 {
    // TODO: call the raw version with this slice's own pointer and length
    todo!()
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
