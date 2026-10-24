/// Takes ownership of the name and says how long it is.
pub fn name_length(name: String) -> usize {
    name.len()
}

/// The length of "ferris" AND the name itself.
pub fn length_and_name() -> (usize, String) {
    let name = String::from("ferris");
    let len = name_length(name.clone());
    (len, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_and_name_returns_both() {
        assert_eq!(length_and_name(), (6, "ferris".to_string()));
    }
}
