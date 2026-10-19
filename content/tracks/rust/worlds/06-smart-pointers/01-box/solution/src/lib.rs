/// A singly linked list of i32. The Box gives the recursive part a known size.
#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

/// Returns a new list with `x` in front of `list`.
pub fn prepend(list: List, x: i32) -> List {
    Cons(x, Box::new(list))
}

/// Adds up every element. An empty list sums to 0.
pub fn sum(list: &List) -> i32 {
    match list {
        Cons(v, rest) => v + sum(rest),
        Nil => 0,
    }
}

/// Counts the elements.
pub fn len(list: &List) -> usize {
    match list {
        Cons(_, rest) => 1 + len(rest),
        Nil => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three() -> List {
        Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
    }

    #[test]
    fn prepend_puts_new_head_in_front() {
        let list = prepend(Cons(2, Box::new(Nil)), 1);
        assert_eq!(list, Cons(1, Box::new(Cons(2, Box::new(Nil)))));
    }

    #[test]
    fn sum_adds_every_element() {
        assert_eq!(sum(&three()), 6);
        assert_eq!(sum(&Nil), 0, "an empty list sums to 0");
    }

    #[test]
    fn len_counts_elements() {
        assert_eq!(len(&three()), 3);
        assert_eq!(len(&Nil), 0, "an empty list has no elements");
    }
}
