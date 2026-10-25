/// `block_names!["stone", "dirt"]` is `vec!["stone".to_string(), "dirt".to_string()]`.
#[macro_export]
macro_rules! block_names {
    ($($s:expr),*) => {
        // TODO: build a vec! that holds `.to_string()` of each captured `$s`, separated by commas.
        // (This stub ignores the arguments, so the crate compiles.)
        Vec::<String>::new()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn block_names_converts_each_argument() {
        let blocks: Vec<String> = block_names!["stone", "dirt"];
        assert_eq!(blocks, vec!["stone".to_string(), "dirt".to_string()]);
    }

    #[test]
    fn block_names_takes_any_count() {
        let one: Vec<String> = block_names!["bedrock"];
        assert_eq!(one, vec!["bedrock".to_string()]);
        let five: Vec<String> = block_names!["stone", "dirt", "gravel", "sand", "oak_log"];
        assert_eq!(five.len(), 5);
        assert_eq!(five[4], "oak_log");
    }

    #[test]
    fn block_names_takes_zero() {
        let none: Vec<String> = block_names![];
        assert!(none.is_empty());
    }
}
