/// `block_names!["stone", "dirt"]` is `vec!["stone".to_string(), "dirt".to_string()]`. A comma after the last
/// block is fine.
#[macro_export]
macro_rules! block_names {
    ($($s:expr),* $(,)?) => {
        vec![$($s.to_string()),*]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn block_names_accepts_a_trailing_comma() {
        let blocks: Vec<String> = block_names![
            "stone",
            "dirt",
        ];
        assert_eq!(blocks, vec!["stone".to_string(), "dirt".to_string()]);
    }

    #[test]
    fn block_names_still_works_without_one() {
        let blocks: Vec<String> = block_names!["sand", "gravel"];
        assert_eq!(blocks.len(), 2);
        let none: Vec<String> = block_names![];
        assert!(none.is_empty());
    }
}
