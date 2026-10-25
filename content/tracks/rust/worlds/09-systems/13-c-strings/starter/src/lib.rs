//! Text that crosses into C and back.

use std::ffi::{CStr, CString, c_char};

unsafe extern "C" {
    /// C's `strlen`: counts the bytes before the first NUL.
    fn strlen(s: *const c_char) -> usize;
}

/// The length of `s` in bytes, counted by C's `strlen`.
/// `None` if `s` contains a NUL byte (it cannot be a C string).
pub fn c_strlen(s: &str) -> Option<usize> {
    // TODO: make a C copy of the text (None if it has a NUL inside), then
    //       let strlen count it through the copy's pointer
    todo!()
}

/// Copies a C string into a Rust `String` (invalid UTF-8 is replaced).
///
/// # Safety
/// `ptr` must be non-null and point to a NUL-terminated string that stays
/// valid for the duration of the call.
pub unsafe fn from_c(ptr: *const c_char) -> String {
    // TODO: borrow the C string behind ptr, then turn it into an owned String
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strlen_wrapper_counts_bytes() {
        assert_eq!(c_strlen("hello"), Some(5));
        assert_eq!(c_strlen(""), Some(0));
        assert_eq!(c_strlen("héllo"), Some(6), "strlen counts bytes, not letters");
    }

    #[test]
    fn strlen_rejects_interior_nul() {
        assert_eq!(c_strlen("ab\0cd"), None);
    }

    #[test]
    fn cstr_converts_back_to_string() {
        let c = CString::new("from C").unwrap();
        // SAFETY: c ends in a NUL and is alive for the call.
        let s = unsafe { from_c(c.as_ptr()) };
        assert_eq!(s, "from C");
    }
}
