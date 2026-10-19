use std::ops::Deref;

/// A box-like wrapper around any value.
pub struct MyBox<T>(pub T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // TODO: return a reference to the wrapped value (field 0)
        todo!()
    }
}

/// Takes a plain &str, so passing &MyBox<String> relies on deref coercion.
pub fn str_len(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_gives_inner_value() {
        let b = MyBox::new(5);
        assert_eq!(*b, 5);
        let s = MyBox::new(String::from("hi"));
        assert_eq!(str_len(&s), 2, "&MyBox<String> should coerce to &str");
    }
}
