/// Adds "!!" to the end of the caller's String.
pub fn add_exclaim(s: &mut String) {
    // TODO: add "!!" to the end of s
    todo!()
}

/// Pushes `x` onto the caller's Vec two times.
pub fn push_twice(v: &mut Vec<i32>, x: i32) {
    // TODO: push x two times
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_exclaim_appends() {
        let mut s = String::from("hey");
        add_exclaim(&mut s);
        assert_eq!(s, "hey!!");
    }

    #[test]
    fn push_twice_adds_two() {
        let mut v = vec![1];
        push_twice(&mut v, 7);
        assert_eq!(v, vec![1, 7, 7]);
    }
}
