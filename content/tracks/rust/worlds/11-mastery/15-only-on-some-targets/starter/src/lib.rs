//! Where the notes app keeps its files. A desktop build writes to a folder; a build for the
//! browser (wasm32) has no folders and uses the browser's storage instead.

// TODO: two functions with one name do not compile. Mark this one so it is only compiled when
// building for wasm32 ...
pub fn save_location() -> &'static str {
    "browser storage"
}

// TODO: ... and this one so it is compiled for every other target.
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
