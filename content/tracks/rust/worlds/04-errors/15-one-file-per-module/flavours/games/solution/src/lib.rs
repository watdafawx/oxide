pub mod manifest;
pub mod resolver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_file_is_wired_in() {
        assert_eq!(manifest::version_of("sodium@5"), 5);
        assert_eq!(manifest::version_of("no version here"), 0);
    }

    #[test]
    fn resolver_adds_up_through_the_manifest() {
        assert_eq!(resolver::pack_fingerprint(&["sodium@5", "lithium@3", "oops"]), 8);
        assert_eq!(resolver::pack_fingerprint(&[]), 0);
    }
}
