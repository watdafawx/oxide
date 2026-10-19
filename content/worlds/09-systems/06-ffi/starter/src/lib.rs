//! Calling the C runtime.

use std::ffi::{CStr, CString, c_char, c_int};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn abs(x: c_int) -> c_int;
}

/// The absolute value of `n`, computed by C's `abs`.
pub fn c_abs(n: i32) -> i32 {
    // TODO: unsafe { abs(n) }  (with a // SAFETY: comment)
    todo!()
}

/// The length of `s` in bytes, computed by C's `strlen`.
/// `None` if `s` contains a NUL byte (it cannot be a C string).
pub fn c_strlen(s: &str) -> Option<usize> {
    // TODO: let c = CString::new(s).ok()?;  then unsafe { strlen(c.as_ptr()) }
    //       (CString::new fails on an interior NUL, which gives None)
    todo!()
}

/// Copies a C string into a Rust `String` (invalid UTF-8 is replaced).
///
/// # Safety
/// `ptr` must be non-null and point to a NUL-terminated string that stays
/// valid for the duration of the call.
pub unsafe fn from_c(ptr: *const c_char) -> String {
    // TODO: CStr::from_ptr(ptr) inside unsafe, then .to_string_lossy().into_owned()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_calls_c_runtime() {
        assert_eq!(c_abs(-5), 5);
        assert_eq!(c_abs(7), 7);
        assert_eq!(c_abs(0), 0);
    }

    #[test]
    fn strlen_wrapper_counts_bytes() {
        assert_eq!(c_strlen("hello"), Some(5));
        assert_eq!(c_strlen(""), Some(0));
        assert_eq!(c_strlen("héllo"), Some(6), "strlen counts bytes, not chars");
    }

    #[test]
    fn strlen_rejects_interior_nul() {
        assert_eq!(c_strlen("ab\0cd"), None);
    }

    #[test]
    fn cstr_converts_back_to_string() {
        let c = CString::new("from C").unwrap();
        // SAFETY: c is NUL-terminated and alive for the call.
        let s = unsafe { from_c(c.as_ptr()) };
        assert_eq!(s, "from C");
    }
}
