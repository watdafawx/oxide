/// "ana" -> "ANA!!"
pub fn shout(name: &str) -> &str {
    let mut loud = name.to_uppercase();
    loud.push_str("!!");
    // TODO: this does not build: `loud` is dropped at the `}`, so a borrow of it would dangle.
    // Hand back the String itself instead.
    &loud
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shout_gives_back_new_text() {
        assert_eq!(shout("ana"), "ANA!!");
        assert_eq!(shout("bo"), "BO!!");
    }
}
