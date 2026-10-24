//! Calling a function from the C standard library.

use std::ffi::c_int;

unsafe extern "C" {
    /// C's `abs`: the distance of `x` from 0.
    fn abs(x: c_int) -> c_int;
}

/// The distance of `n` from 0, computed by C's `abs`.
pub fn c_abs(n: i32) -> i32 {
    // SAFETY: abs is defined for every int except the smallest one, and the
    //         tests stay far from it.
    unsafe { abs(n) }
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
}
