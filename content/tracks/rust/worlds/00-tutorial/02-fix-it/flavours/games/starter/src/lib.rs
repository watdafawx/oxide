/// The longest player name a name tag can show.
pub fn max_name_len() -> usize {
    // TODO: this is off by one
    15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_limit_is_sixteen() {
        assert_eq!(max_name_len(), 16);
    }
}
