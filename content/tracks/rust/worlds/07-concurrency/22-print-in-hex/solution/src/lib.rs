use std::path::Path;

/// One line of the report: the hash as 16 lowercase hex digits (zero padded),
/// two spaces, then the path.
pub fn hex_line(path: &Path, hash: u64) -> String {
    format!("{hash:016x}  {}", path.display())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_line_pads_to_sixteen_digits() {
        assert_eq!(hex_line(Path::new("a.txt"), 0xff), "00000000000000ff  a.txt");
        assert_eq!(
            hex_line(Path::new("photos/cat.jpg"), 0xaf63dc4c8601ec8c),
            "af63dc4c8601ec8c  photos/cat.jpg"
        );
    }
}
