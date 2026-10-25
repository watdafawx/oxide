use std::borrow::Cow;

/// Replaces each tab with a single space. Borrows the input when there is nothing to change.
pub fn normalise(input: &str) -> Cow<'_, str> {
    if input.contains('\t') {
        Cow::Owned(input.replace('\t', " "))
    } else {
        Cow::Borrowed(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_input_is_borrowed() {
        let out = normalise("no tabs here");
        assert!(
            matches!(out, Cow::Borrowed(_)),
            "nothing changed, so nothing should be allocated"
        );
    }

    #[test]
    fn tabs_become_single_spaces() {
        assert_eq!(normalise("a\tb\t\tc"), "a b  c");
    }

    #[test]
    fn changed_input_is_owned() {
        let out = normalise("x\ty");
        assert!(matches!(out, Cow::Owned(_)), "a changed string must be Owned");
    }
}
