//! Raw pointers and a safe wrapper.

/// Adds 1 to the i32 `ptr` points at.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to a live, writable i32 that
/// nothing else is accessing.
pub unsafe fn increment(ptr: *mut i32) {
    // SAFETY: the caller guarantees `ptr` is valid for writes.
    unsafe {
        *ptr += 1;
    }
}

/// Reads the i32 behind `ptr`, or `None` if `ptr` is null.
///
/// # Safety
/// If `ptr` is not null it must be aligned and point to a live i32.
pub unsafe fn read_or_none(ptr: *const i32) -> Option<i32> {
    if ptr.is_null() {
        None
    } else {
        // SAFETY: non-null, and the caller guarantees the rest.
        Some(unsafe { *ptr })
    }
}

/// Sums `len` i32s starting at `ptr`.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to `len` consecutive live i32s
/// that are not mutated while this runs.
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    // SAFETY: the caller guarantees ptr/len describe a valid region.
    let items = unsafe { std::slice::from_raw_parts(ptr, len) };
    items.iter().sum()
}

/// Sums a slice. Safe to call with any slice.
pub fn safe_sum(slice: &[i32]) -> i32 {
    // SAFETY: a slice's pointer and length always describe a valid region.
    unsafe { sum_raw(slice.as_ptr(), slice.len()) }
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
