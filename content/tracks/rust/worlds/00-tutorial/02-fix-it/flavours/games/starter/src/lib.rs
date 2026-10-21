/// The longest player name the server accepts.
pub fn max_name_len() -> usize {
    // TODO: this is off by one, read the test's `right` side
    15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usernames_fit_sixteen_letters() {
        assert_eq!(max_name_len(), 16);
    }
}
