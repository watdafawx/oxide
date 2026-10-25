//! The chunk indexer's core, shared by the desktop tool and the browser map viewer.

/// The index of chunk (x, z) inside its 32 by 32 region: x + z * 32.
// TODO: above `pub fn chunk_index`, add the attribute that applies `wasm_bindgen` (written as the
// full path wasm_bindgen::prelude::wasm_bindgen) only when building for wasm32.
pub fn chunk_index(x: u32, z: u32) -> u32 {
    // TODO: return x + z * 32
    let _ = (x, z);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_index_counts_rows_of_32() {
        assert_eq!(chunk_index(0, 0), 0);
        assert_eq!(chunk_index(5, 1), 37);
        assert_eq!(chunk_index(31, 31), 1023);
    }

    #[test]
    fn chunk_index_is_exported_for_wasm() {
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
            compact.contains("#[cfg_attr(target_arch=\"wasm32\",wasm_bindgen::prelude::wasm_bindgen)]pubfnchunk_index"),
            "put `#[cfg_attr(target_arch = \"wasm32\", wasm_bindgen::prelude::wasm_bindgen)]` right above `pub fn chunk_index`"
        );
    }
}
