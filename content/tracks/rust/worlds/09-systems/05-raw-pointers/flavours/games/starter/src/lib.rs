//! Raw pointers and a safe wrapper, for player counts.

/// Adds 1 to the player count `ptr` points at.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to a live, writable i32 that
/// nothing else is accessing.
pub unsafe fn add_player(ptr: *mut i32) {
    // TODO: unsafe { *ptr += 1; } with a // SAFETY: comment
    todo!()
}

/// Reads the player count behind `ptr`, or `None` if `ptr` is null.
///
/// # Safety
/// If `ptr` is not null it must be aligned and point to a live i32.
pub unsafe fn player_count_or_none(ptr: *const i32) -> Option<i32> {
    // TODO: is_null() check, then dereference inside unsafe
    todo!()
}

/// Sums `len` player counts starting at `ptr`.
///
/// # Safety
/// `ptr` must be non-null, aligned, and point to `len` consecutive live i32s
/// that are not mutated while this runs.
pub unsafe fn total_players_raw(ptr: *const i32, len: usize) -> i32 {
    // TODO: std::slice::from_raw_parts(ptr, len) in an unsafe block, then sum
    todo!()
}

/// Sums a slice of player counts. Safe to call with any slice.
pub fn total_players(counts: &[i32]) -> i32 {
    // TODO: call total_players_raw with counts.as_ptr() and counts.len()
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

    #[test]
    fn sums_players_from_raw_parts() {
        let v = [10, 20, 30, 40];
        // SAFETY: pointer and lengths are within `v`.
        unsafe {
            assert_eq!(total_players_raw(v.as_ptr(), 4), 100);
            assert_eq!(total_players_raw(v.as_ptr().add(2), 2), 70, "start at the third server");
            assert_eq!(total_players_raw(v.as_ptr(), 0), 0);
        }
    }

    #[test]
    fn safe_wrapper_totals_a_slice() {
        assert_eq!(total_players(&[1, 2, 3]), 6);
        assert_eq!(total_players(&[]), 0, "an empty slice must work too");
    }
}
