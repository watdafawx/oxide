//! A server MOTD that crosses into C and back.

use std::ffi::{CStr, CString, c_char};

unsafe extern "C" {
    /// C's `strlen`: counts the bytes before the first NUL.
    fn strlen(s: *const c_char) -> usize;
}

/// The length of `motd` in bytes, counted by C's `strlen`.
/// `None` if `motd` contains a NUL byte (it cannot be a C string).
pub fn c_motd_len(motd: &str) -> Option<usize> {
    let c = CString::new(motd).ok()?;
    // SAFETY: `c` ends in a NUL and lives until the end of this function.
    Some(unsafe { strlen(c.as_ptr()) })
}

/// Copies a C string into a Rust `String` (invalid UTF-8 is replaced).
///
/// # Safety
/// `ptr` must be non-null and point to a NUL-terminated string that stays
/// valid for the duration of the call.
pub unsafe fn motd_from_c(ptr: *const c_char) -> String {
    // SAFETY: guaranteed by the caller.
    let c = unsafe { CStr::from_ptr(ptr) };
    c.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_motd_len_counts_bytes() {
        assert_eq!(c_motd_len("hello"), Some(5));
        assert_eq!(c_motd_len(""), Some(0));
        assert_eq!(c_motd_len("Café"), Some(5), "strlen counts bytes, not letters");
    }

    #[test]
    fn c_motd_len_rejects_interior_nul() {
        assert_eq!(c_motd_len("ab\0cd"), None);
    }

    #[test]
    fn motd_from_c_reads_back_a_string() {
        let c = CString::new("A Minecraft Server").unwrap();
        // SAFETY: c ends in a NUL and is alive for the call.
        let s = unsafe { motd_from_c(c.as_ptr()) };
        assert_eq!(s, "A Minecraft Server");
    }
}
