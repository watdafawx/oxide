//! Raw pointers and a safe wrapper.

/// Adds 1 to the i32 `ptr` points at.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to a live, writable i32 that
/// nothing else is accessing.
pub unsafe fn increment(ptr: *mut i32) {
    // TODO: add 1 through the pointer inside an unsafe block (with a // SAFETY: comment)
    todo!()
}

/// Reads the i32 behind `ptr`, or `None` if `ptr` is null.
///
/// # Safety
/// If `ptr` is not null it must be aligned and point to a live i32.
pub unsafe fn read_or_none(ptr: *const i32) -> Option<i32> {
    // TODO: None for a null pointer; otherwise read the value through it (only the read needs unsafe)
    todo!()
}

/// Sums `len` i32s starting at `ptr`.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to `len` consecutive live i32s
/// that are not mutated while this runs.
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    // TODO: rebuild a slice from the pointer and the length (inside unsafe),
    //       then sum it
    todo!()
}

/// Sums a slice. Safe to call with any slice.
pub fn safe_sum(slice: &[i32]) -> i32 {
    // TODO: call sum_raw with the slice's pointer and length inside unsafe,
    //       with a // SAFETY: comment saying why a slice is always valid
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_through_mut_pointer() {
        let mut x = 41;
        // SAFETY: &mut x is a valid, exclusive pointer.
        unsafe { increment(&mut x) };
        assert_eq!(x, 42);
    }

    #[test]
    fn null_pointer_gives_none() {
        let x = 7;
        // SAFETY: &x is valid; a null pointer is explicitly allowed.
        unsafe {
            assert_eq!(read_or_none(std::ptr::null()), None);
            assert_eq!(read_or_none(&x), Some(7));
        }
    }

    #[test]
    fn sums_raw_parts() {
        let v = [10, 20, 30, 40];
        // SAFETY: pointer and lengths are within `v`.
        unsafe {
            assert_eq!(sum_raw(v.as_ptr(), 4), 100);
            assert_eq!(sum_raw(v.as_ptr().add(2), 2), 70, "start at the third item");
            assert_eq!(sum_raw(v.as_ptr(), 0), 0);
        }
    }

    #[test]
    fn safe_wrapper_sums_slice() {
        assert_eq!(safe_sum(&[1, 2, 3]), 6);
        assert_eq!(safe_sum(&[]), 0, "an empty slice must work too");
    }
}
