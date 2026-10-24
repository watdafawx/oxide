/// "ana" -> "ANA!!"
pub fn shout(name: &str) -> String {
    let mut loud = name.to_uppercase();
    loud.push_str("!!");
    loud
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
