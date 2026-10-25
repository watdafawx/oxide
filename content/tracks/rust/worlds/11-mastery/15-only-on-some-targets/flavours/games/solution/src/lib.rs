//! Where the chunk indexer keeps its cache. A desktop build writes to the saves folder; a build
//! for the browser map viewer (wasm32) has no folders and uses the browser's storage instead.

#[cfg(target_arch = "wasm32")]
pub fn save_location() -> &'static str {
    "browser storage"
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_location() -> &'static str {
    ".minecraft/saves/"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_build_saves_to_the_saves_folder() {
        assert_eq!(save_location(), ".minecraft/saves/");
    }
}
