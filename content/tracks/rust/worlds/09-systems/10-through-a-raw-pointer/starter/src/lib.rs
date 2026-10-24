//! Reading and writing through raw pointers.

/// Adds 1 to the number `ptr` points at.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to a live, writable i32 that
/// nothing else is accessing.
pub unsafe fn increment(ptr: *mut i32) {
    // TODO: add 1 to the value behind the pointer (an unsafe block, with a
    //       // SAFETY: comment saying why it is fine)
    todo!()
}

/// Reads the number behind `ptr`, or `None` if `ptr` is null.
///
/// # Safety
/// If `ptr` is not null it must be aligned and point to a live i32.
pub unsafe fn read_or_none(ptr: *const i32) -> Option<i32> {
    // TODO: None for a null pointer; otherwise the value behind it
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
}
