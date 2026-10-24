//! Calling a function from the C standard library.

use std::ffi::c_int;

unsafe extern "C" {
    /// C's `abs`: the distance of `x` from 0.
    fn abs(x: c_int) -> c_int;
}

/// How far `n` blocks is from spawn (0), computed by C's `abs`.
pub fn c_abs(n: i32) -> i32 {
    // TODO: call the C function (calling it is unsafe: add a // SAFETY: comment)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_abs_gives_distance_from_spawn() {
        assert_eq!(c_abs(-5), 5, "5 blocks west of spawn is 5 blocks away");
        assert_eq!(c_abs(7), 7);
        assert_eq!(c_abs(0), 0);
    }
}
