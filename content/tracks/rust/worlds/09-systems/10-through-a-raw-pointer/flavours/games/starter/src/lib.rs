//! Reading and writing player counts through raw pointers.

/// Adds 1 to the player count `ptr` points at.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to a live, writable i32 that
/// nothing else is accessing.
pub unsafe fn add_player(ptr: *mut i32) {
    // TODO: add 1 to the value behind the pointer (an unsafe block, with a
    //       // SAFETY: comment saying why it is fine)
    todo!()
}

/// Reads the player count behind `ptr`, or `None` if `ptr` is null.
///
/// # Safety
/// If `ptr` is not null it must be aligned and point to a live i32.
pub unsafe fn player_count_or_none(ptr: *const i32) -> Option<i32> {
    // TODO: None for a null pointer; otherwise the value behind it
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_a_player_through_mut_pointer() {
        let mut players = 41;
        // SAFETY: &mut players is a valid, exclusive pointer.
        unsafe { add_player(&mut players) };
        assert_eq!(players, 42);
    }

    #[test]
    fn null_pointer_gives_none() {
        let players = 7;
        // SAFETY: &players is valid; a null pointer is explicitly allowed.
        unsafe {
            assert_eq!(player_count_or_none(std::ptr::null()), None);
            assert_eq!(player_count_or_none(&players), Some(7));
        }
    }
}
