//! Where the notes app keeps its files. A desktop build writes to a folder; a build for the
//! browser (wasm32) has no folders and uses the browser's storage instead.

#[cfg(target_arch = "wasm32")]
pub fn save_location() -> &'static str {
    "browser storage"
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_location() -> &'static str {
    "notes/"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_build_saves_to_a_folder() {
        assert_eq!(save_location(), "notes/");
    }
}
