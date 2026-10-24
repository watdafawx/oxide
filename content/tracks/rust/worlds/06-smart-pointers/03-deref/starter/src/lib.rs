use std::ops::Deref;

/// A home-made box: it owns one value and acts like a reference to it.
pub struct MyBox<T>(pub T);

impl<T> MyBox<T> {
    pub fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // TODO: return a reference to the wrapped value (field 0)
        todo!()
    }
}

/// Takes a plain &str, so passing a &MyBox<String> relies on deref coercion.
pub fn str_len(text: &str) -> usize {
    text.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_reads_the_value_inside() {
        let b = MyBox::new(5);
        assert_eq!(*b, 5);
    }

    #[test]
    fn my_box_of_string_works_as_str() {
        let title = MyBox::new(String::from("Groceries"));
        assert_eq!(str_len(&title), 9, "&MyBox<String> should coerce to &str");
    }
}
