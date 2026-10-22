//! Kinetik Stats, rebuilt in Rust.

/// The name on Kinetik's window.
pub fn app_title() -> String {
    "Kinetik Stats".to_string()
}

#[cfg(test)]
mod tests {
    /// Reads Cargo.toml itself and looks for the gpui line under [dependencies].
    #[test]
    fn gpui_is_in_dependencies() {
        let manifest = include_str!("../Cargo.toml");
        let deps = manifest.find("[dependencies]").expect("Cargo.toml has a [dependencies] table");
        let line = manifest[deps..]
            .lines()
            .find(|l| l.trim_start().starts_with("gpui"))
            .expect("no `gpui = ...` line under [dependencies] yet");
        assert!(line.contains("\"=0.2.2\""), "pin the version: \"=0.2.2\", got: {line}");
        assert!(line.contains("test-support"), "add features = [\"test-support\"], got: {line}");
    }
}
