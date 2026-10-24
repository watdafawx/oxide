//! The text tools' core, shared by the desktop app and a web page.

/// How many words `text` has: runs of characters between whitespace.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn word_count(text: &str) -> u32 {
    text.split_whitespace().count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_count_counts_words() {
        assert_eq!(word_count("the quick  brown fox"), 4);
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("  one  "), 1);
    }

    #[test]
    fn word_count_is_exported_for_wasm() {
        // This sandbox cannot build for the browser, so this test reads the source above the
        // tests instead, ignoring comments and spaces.
        let source = include_str!("lib.rs");
        let code = source.split("#[cfg(test)]").next().unwrap();
        let compact: String = code
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .flat_map(|l| l.chars().filter(|c| !c.is_whitespace()))
            .collect();
        assert!(
            compact.contains("#[cfg_attr(target_arch=\"wasm32\",wasm_bindgen::prelude::wasm_bindgen)]pubfnword_count"),
            "put `#[cfg_attr(target_arch = \"wasm32\", wasm_bindgen::prelude::wasm_bindgen)]` right above `pub fn word_count`"
        );
    }
}
