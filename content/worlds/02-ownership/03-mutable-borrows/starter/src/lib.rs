/// Adds a '!' to the end of the caller's String.
pub fn add_exclaim(s: &mut String) {
    // TODO: push a '!' onto s
    todo!()
}

/// Pushes `x` onto the Vec two times.
pub fn push_twice(v: &mut Vec<i32>, x: i32) {
    // TODO: push x two times
    todo!()
}

/// Moves the last element of `from` to the end of `to`. Does nothing if `from` is empty.
pub fn move_last(from: &mut Vec<i32>, to: &mut Vec<i32>) {
    // TODO: pop from `from`; if there was a value, push it onto `to`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_exclaim_appends() {
        let mut s = String::from("hey");
        add_exclaim(&mut s);
        assert_eq!(s, "hey!");
    }

    #[test]
    fn push_twice_adds_two() {
        let mut v = vec![1];
        push_twice(&mut v, 7);
        assert_eq!(v, vec![1, 7, 7]);
    }

    #[test]
    fn move_last_between_vecs() {
        let mut a = vec![1, 2, 3];
        let mut b = vec![9];
        move_last(&mut a, &mut b);
        assert_eq!((a, b), (vec![1, 2], vec![9, 3]));
        let mut empty = Vec::new();
        let mut c = vec![5];
        move_last(&mut empty, &mut c);
        assert_eq!(c, vec![5], "nothing to move from an empty Vec");
    }
}
