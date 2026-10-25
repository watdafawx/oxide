use std::path::Path;

/// One line of the report: the hash as 16 lowercase hex digits (zero padded),
/// two spaces, then the path.
pub fn hex_line(path: &Path, hash: u64) -> String {
    // TODO: format the hash as 16 zero-padded lowercase hex digits, two spaces, path.display()
    format!("{hash}  {}", path.display())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_line_pads_to_sixteen_digits() {
        assert_eq!(hex_line(Path::new("options.txt"), 0xff), "00000000000000ff  options.txt");
        assert_eq!(
            hex_line(Path::new("mods/sodium.jar"), 0xaf63dc4c8601ec8c),
            "af63dc4c8601ec8c  mods/sodium.jar"
        );
    }
}
