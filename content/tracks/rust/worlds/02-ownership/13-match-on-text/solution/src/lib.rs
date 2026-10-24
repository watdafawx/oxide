/// "admin" -> 3, "mod" -> 2, anything else -> 1.
pub fn rank_owned(name: String) -> u32 {
    match name.as_str() {
        "admin" => 3,
        "mod" => 2,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_owned_matches_on_as_str() {
        assert_eq!(rank_owned("admin".to_string()), 3);
        assert_eq!(rank_owned("mod".to_string()), 2);
        assert_eq!(rank_owned("guest".to_string()), 1);
    }
}
