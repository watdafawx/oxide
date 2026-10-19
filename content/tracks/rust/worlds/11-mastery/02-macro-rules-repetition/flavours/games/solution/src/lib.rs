/// `block_names!["stone", "dirt"]` gives `vec!["stone".to_string(), "dirt".to_string()]`.
#[macro_export]
macro_rules! block_names {
    ($($s:expr),* $(,)?) => {
        vec![$($s.to_string()),*]
    };
}

/// `block_ids!("stone" => 1)` builds a HashMap with one pair.
#[macro_export]
macro_rules! block_ids {
    ($k:expr => $v:expr) => {{
        let mut m = ::std::collections::HashMap::new();
        m.insert($k, $v);
        m
    }};
}

/// `tallest!(3, 9, 2)` is 9.
#[macro_export]
macro_rules! tallest {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {
        ::std::cmp::max($x, $crate::tallest!($($rest),+))
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn block_names_converts_each() {
        let v = block_names!["stone", "dirt"];
        assert_eq!(v, vec!["stone".to_string(), "dirt".to_string()]);
    }

    #[test]
    fn block_names_allows_empty_and_trailing_comma() {
        let empty: Vec<String> = block_names![];
        assert!(empty.is_empty());
        let v = block_names!["gravel", "sand",];
        assert_eq!(v, vec!["gravel".to_string(), "sand".to_string()]);
    }

    #[test]
    fn block_ids_builds_one_pair() {
        let m: HashMap<&str, i32> = block_ids!("stone" => 1);
        assert_eq!(m.len(), 1);
        assert_eq!(m["stone"], 1);
    }

    #[test]
    fn tallest_of_many_heights() {
        assert_eq!(tallest!(5), 5);
        assert_eq!(tallest!(3, 9, 2), 9);
        assert_eq!(tallest!(1, 2, 3, 4, 8, 7, 6), 8);
    }
}
