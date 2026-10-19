/// `vec_of_strings!["a", "b"]` gives `vec!["a".to_string(), "b".to_string()]`.
#[macro_export]
macro_rules! vec_of_strings {
    ($($s:expr),* $(,)?) => {{
        // TODO: build the Vec<String> with `vec![$($s.to_string()),*]`.
        // (This stub ignores the arguments, so the crate compiles.)
        $(let _ = $s;)*
        Vec::<String>::new()
    }};
}

/// `max!(3, 9, 2)` is 9.
#[macro_export]
macro_rules! max {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {{
        // TODO: return `::std::cmp::max($x, max!($($rest),+))`: the macro calls itself on the rest.
        // (This stub ignores the arguments, so the crate compiles.)
        let _ = ($x, $($rest),+);
        0
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn vec_of_strings_converts_each() {
        let v = vec_of_strings!["a", "b"];
        assert_eq!(v, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn vec_of_strings_allows_empty_and_trailing_comma() {
        let empty: Vec<String> = vec_of_strings![];
        assert!(empty.is_empty());
        let v = vec_of_strings!["x", "y",];
        assert_eq!(v, vec!["x".to_string(), "y".to_string()]);
    }

    #[test]
    fn max_of_many_arguments() {
        assert_eq!(max!(5), 5);
        assert_eq!(max!(3, 9, 2), 9);
        assert_eq!(max!(1, 2, 3, 4, 8, 7, 6), 8);
    }
}
