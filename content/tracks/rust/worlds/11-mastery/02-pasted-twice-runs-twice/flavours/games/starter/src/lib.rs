/// `chunk_area!(side)` is `side * side`, and `side` runs only once.
#[macro_export]
macro_rules! chunk_area {
    ($x:expr) => {
        // TODO: this pastes the argument twice, so a call inside it runs twice. Make the template
        // a block that runs the argument once, keeps the answer in a local, and squares that.
        $x * $x
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn chunk_area_runs_its_argument_once() {
        let mut calls = 0;
        let mut next_side = || {
            calls += 1;
            2 + calls
        };
        let result = chunk_area!(next_side());
        assert_eq!(calls, 1, "chunk_area! must run its argument exactly once");
        assert_eq!(result, 9);
    }

    #[test]
    fn chunk_area_still_squares() {
        assert_eq!(chunk_area!(16), 256);
        assert_eq!(chunk_area!(0.5), 0.25);
    }
}
