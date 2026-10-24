/// `block_names!["stone", "dirt"]` gives `vec!["stone".to_string(), "dirt".to_string()]`.
#[macro_export]
macro_rules! block_names {
    // TODO: expand to a vec! that repeats `.to_string()` on each `$s` (the pattern below is already right)
    ($($s:expr),* $(,)?) => {
        Vec::<String>::new()
    };
}

/// `block_ids!("stone" => 1)` builds a HashMap with one pair.
#[macro_export]
macro_rules! block_ids {
    // TODO: create a mutable HashMap (full path from ::std), insert the key and the value, then end with the map
    ($k:expr => $v:expr) => {{
        let _ = ($k, $v);
        ::std::collections::HashMap::new()
    }};
}

/// `tallest!(3, 9, 2)` is 9.
#[macro_export]
macro_rules! tallest {
    // TODO: two arms: one argument gives itself back; one argument plus a repeated rest gives the
    // larger of the first and tallest! of the rest (call it through $crate)
    ($($x:expr),+) => {
        0
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
