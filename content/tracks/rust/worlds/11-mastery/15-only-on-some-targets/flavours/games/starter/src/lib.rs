//! Where the chunk indexer keeps its cache. A desktop build writes to the saves folder; a build
//! for the browser map viewer (wasm32) has no folders and uses the browser's storage instead.

// TODO: two functions with one name do not compile. Mark this one so it is only compiled when
// building for wasm32 ...
pub fn save_location() -> &'static str {
    "browser storage"
}

// TODO: ... and this one so it is compiled for every other target.
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
