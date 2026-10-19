/// The answer to life, the universe and everything.
pub fn answer() -> u32 {
    // TODO: this is off by one
    41
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_is_right() {
        assert_eq!(answer(), 42);
    }
}
