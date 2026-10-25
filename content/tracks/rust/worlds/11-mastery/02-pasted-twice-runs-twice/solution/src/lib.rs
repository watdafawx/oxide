/// `square!(x)` is `x * x`, and `x` runs only once.
#[macro_export]
macro_rules! square {
    ($x:expr) => {{
        let v = $x;
        v * v
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn square_runs_its_argument_once() {
        let mut calls = 0;
        let mut next = || {
            calls += 1;
            2 + calls
        };
        let result = square!(next());
        assert_eq!(calls, 1, "square! must run its argument exactly once");
        assert_eq!(result, 9);
    }

    #[test]
    fn square_still_squares() {
        assert_eq!(square!(4), 16);
        assert_eq!(square!(1.5), 2.25);
    }
}
