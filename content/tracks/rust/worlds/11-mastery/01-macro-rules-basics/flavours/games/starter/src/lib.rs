/// `chunk_area!(x)` expands to `x * x`.
#[macro_export]
macro_rules! chunk_area {
    // TODO: ($x:expr) => {{ ... }}; evaluate $x once
    ($x:expr) => {
        0 * $x
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn chunk_area_multiplies() {
        assert_eq!(chunk_area!(4), 16);
        assert_eq!(chunk_area!(1 + 2), 9, "`1 + 2` is one expression: (1 + 2) * (1 + 2)");
    }

    #[test]
    fn chunk_area_evaluates_once() {
        let mut calls = 0;
        let mut next = || {
            calls += 1;
            2 + calls
        };
        let result = chunk_area!(next());
        assert_eq!(result, 9);
        assert_eq!(calls, 1, "the argument must be evaluated exactly once");
    }
}
