use std::borrow::Cow;

/// Puts the mod id in front of a bare item name. Borrows the name when it already has one.
pub fn qualified_name<'a>(name: &'a str, mod_id: &str) -> Cow<'a, str> {
    if name.contains(':') {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(format!("{mod_id}:{name}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixed_name_is_borrowed() {
        let out = qualified_name("krastorio:gear-wheel", "krastorio");
        assert!(
            matches!(out, Cow::Borrowed(_)),
            "nothing changed, so nothing should be allocated"
        );
    }

    #[test]
    fn bare_name_gets_mod_prefix() {
        assert_eq!(qualified_name("gear-wheel", "krastorio"), "krastorio:gear-wheel");
        assert_eq!(qualified_name("krastorio:gear-wheel", "krastorio"), "krastorio:gear-wheel");
    }

    #[test]
    fn prefixed_bare_name_is_owned() {
        let out = qualified_name("gear-wheel", "krastorio");
        assert!(matches!(out, Cow::Owned(_)), "a prefixed name must be Owned");
    }
}
