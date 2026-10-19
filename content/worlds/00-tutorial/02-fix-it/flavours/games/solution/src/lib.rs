/// The longest player name a name tag can show.
pub fn max_name_len() -> usize {
    16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_limit_is_sixteen() {
        assert_eq!(max_name_len(), 16);
    }
}
