/// "owner" -> 3, "op" -> 2, anything else -> 1.
pub fn permission_level(role: String) -> u32 {
    match role.as_str() {
        "owner" => 3,
        "op" => 2,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_level_matches_role_text() {
        assert_eq!(permission_level("owner".to_string()), 3);
        assert_eq!(permission_level("op".to_string()), 2);
        assert_eq!(permission_level("guest".to_string()), 1);
    }
}
