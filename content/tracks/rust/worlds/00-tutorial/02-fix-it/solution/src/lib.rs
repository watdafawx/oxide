/// The most letters a name tag can show.
pub fn max_tag_len() -> usize {
    20
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_tag_fits_twenty_letters() {
        assert_eq!(max_tag_len(), 20);
    }
}
