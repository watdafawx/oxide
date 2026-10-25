use std::fmt::Display;

/// The value as uppercase text with a "!" at the end: shout("hi") is "HI!".
pub fn shout(x: impl Display) -> String {
    // TODO: turn x into text, uppercase it, add "!"
    let _ = x;
    String::new()
}

/// The even numbers below `limit`: 0, 2, 4, ...
pub fn evens(limit: u32) -> impl Iterator<Item = u32> {
    // TODO: a lazy iterator over 0..limit that keeps the even numbers
    let _ = limit;
    std::iter::empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shout_accepts_anything_displayable() {
        assert_eq!(shout("hi"), "HI!");
        assert_eq!(shout(42), "42!");
        assert_eq!(shout(String::from("ok")), "OK!");
    }

    #[test]
    fn evens_returns_an_iterator() {
        let v: Vec<u32> = evens(7).collect();
        assert_eq!(v, vec![0, 2, 4, 6]);
        assert_eq!(evens(7).count(), 4);
    }
}
