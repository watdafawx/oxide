//! Calling the C runtime on server MOTDs.

use std::ffi::{CStr, CString, c_char, c_int};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn abs(x: c_int) -> c_int;
}

/// The absolute value of `n` (a distance from spawn), computed by C's `abs`.
pub fn c_abs(n: i32) -> i32 {
    // SAFETY: abs has no preconditions beyond a valid int.
    unsafe { abs(n) }
}

/// The length of `motd` in bytes, computed by C's `strlen`.
/// `None` if `motd` contains a NUL byte (it cannot be a C string).
pub fn c_motd_len(motd: &str) -> Option<usize> {
    let c = CString::new(motd).ok()?;
    // SAFETY: `c` is NUL-terminated and lives until the end of this function.
    Some(unsafe { strlen(c.as_ptr()) })
}

/// Copies a C string into a Rust `String` (invalid UTF-8 is replaced).
///
/// # Safety
/// `ptr` must be non-null and point to a NUL-terminated string that stays
/// valid for the duration of the call.
pub unsafe fn motd_from_c(ptr: *const c_char) -> String {
    // SAFETY: guaranteed by the caller.
    unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_abs_gives_distance_from_spawn() {
        assert_eq!(c_abs(-5), 5);
        assert_eq!(c_abs(7), 7);
        assert_eq!(c_abs(0), 0);
    }

    #[test]
    fn c_motd_len_counts_bytes() {
        assert_eq!(c_motd_len("hello"), Some(5));
        assert_eq!(c_motd_len(""), Some(0));
        assert_eq!(c_motd_len("Café"), Some(5), "strlen counts bytes, not chars");
    }

    #[test]
    fn c_motd_len_rejects_interior_nul() {
        assert_eq!(c_motd_len("ab\0cd"), None);
    }

    #[test]
    fn motd_from_c_reads_back_a_string() {
        let c = CString::new("A Minecraft Server").unwrap();
        // SAFETY: c is NUL-terminated and alive for the call.
        let s = unsafe { motd_from_c(c.as_ptr()) };
        assert_eq!(s, "A Minecraft Server");
    }
}
