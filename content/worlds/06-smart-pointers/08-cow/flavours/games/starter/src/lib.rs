use std::borrow::Cow;

/// Puts the mod id in front of a bare item name. Borrows the name when it already has one.
pub fn qualified_name<'a>(name: &'a str, mod_id: &str) -> Cow<'a, str> {
    // TODO: if name contains ':' return Cow::Borrowed(name),
    // otherwise Cow::Owned(format!("{mod_id}:{name}"))
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_input_is_borrowed() {
        let out = qualified_name("create:cogwheel", "create");
        assert!(
            matches!(out, Cow::Borrowed(_)),
            "nothing changed, so nothing should be allocated"
        );
    }

    #[test]
    fn tabs_become_single_spaces() {
        assert_eq!(qualified_name("cogwheel", "create"), "create:cogwheel");
        assert_eq!(qualified_name("create:cogwheel", "create"), "create:cogwheel");
    }

    #[test]
    fn changed_input_is_owned() {
        let out = qualified_name("cogwheel", "create");
        assert!(matches!(out, Cow::Owned(_)), "a prefixed name must be Owned");
    }
}
