/// The longer of the two texts (the first one if they are the same length).
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_returns_the_longer() {
        assert_eq!(longest("hi", "hello"), "hello");
        assert_eq!(longest("hello", "hi"), "hello");
    }

    #[test]
    fn tie_returns_first() {
        let a = String::from("abc");
        let b = String::from("xyz");
        let r = longest(&a, &b);
        assert_eq!(r.as_ptr(), a.as_ptr(), "on a tie the first argument wins");
    }

    #[test]
    fn result_usable_while_inputs_live() {
        let x = String::from("rustacean");
        let y = String::from("crab");
        let r = longest(&x, &y);
        assert_eq!(r, "rustacean");
        assert_eq!(x.len() + y.len(), 13);
    }
}
