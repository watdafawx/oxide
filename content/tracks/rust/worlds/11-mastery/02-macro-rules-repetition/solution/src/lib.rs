/// `vec_of_strings!["a", "b"]` gives `vec!["a".to_string(), "b".to_string()]`.
#[macro_export]
macro_rules! vec_of_strings {
    ($($s:expr),* $(,)?) => {
        vec![$($s.to_string()),*]
    };
}

/// `hashmap!("a" => 1)` builds a HashMap with one pair.
#[macro_export]
macro_rules! hashmap {
    ($k:expr => $v:expr) => {{
        let mut m = ::std::collections::HashMap::new();
        m.insert($k, $v);
        m
    }};
}

/// `max!(3, 9, 2)` is 9.
#[macro_export]
macro_rules! max {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {
        ::std::cmp::max($x, $crate::max!($($rest),+))
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
    fn hashmap_builds_one_pair() {
        let m: HashMap<&str, i32> = hashmap!("a" => 1);
        assert_eq!(m.len(), 1);
        assert_eq!(m["a"], 1);
    }

    #[test]
    fn max_of_many_arguments() {
        assert_eq!(max!(5), 5);
        assert_eq!(max!(3, 9, 2), 9);
        assert_eq!(max!(1, 2, 3, 4, 8, 7, 6), 8);
    }
}
