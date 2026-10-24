/// The longer of two titles (the first one if they are the same length).
// TODO: this does not build: Rust can't tell which input the answer borrows from.
// Add a lifetime `'a` and use it on both inputs and on the result.
pub fn longer(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longer_returns_the_longer() {
        assert_eq!(longer("jam", "bread"), "bread");
        assert_eq!(longer("bread", "jam"), "bread");
    }

    #[test]
    fn tie_returns_first() {
        let a = String::from("abc");
        let b = String::from("xyz");
        let r = longer(&a, &b);
        assert_eq!(r.as_ptr(), a.as_ptr(), "on a tie the first title wins");
    }
}
