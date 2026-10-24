/// `square!(x)` expands to `x * x`.
#[macro_export]
macro_rules! square {
    ($x:expr) => {
        // TODO: make the arm body one block (double braces), run `$x` only once by binding it to a
        // local, and multiply that local by itself.
        // (This stub just returns `$x`, so the crate compiles.)
        $x
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn square_multiplies() {
        assert_eq!(square!(4), 16);
        assert_eq!(square!(1 + 2), 9, "`1 + 2` is one expression: (1 + 2) * (1 + 2)");
    }

    #[test]
    fn square_evaluates_once() {
        let mut calls = 0;
        let mut next = || {
            calls += 1;
            2 + calls
        };
        let result = square!(next());
        assert_eq!(result, 9);
        assert_eq!(calls, 1, "the argument must be evaluated exactly once");
    }
}
