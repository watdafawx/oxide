use std::ops::Deref;

/// A handle to a machine. It acts like the machine inside.
pub struct InserterHandle<T>(pub T);

impl<T> InserterHandle<T> {
    pub fn new(machine: T) -> InserterHandle<T> {
        InserterHandle(machine)
    }
}

impl<T> Deref for InserterHandle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// Takes a plain &str, so passing &InserterHandle<String> relies on deref coercion.
pub fn label_len(label: &str) -> usize {
    label.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_derefs_to_inserter_value() {
        let b = InserterHandle::new(40);
        assert_eq!(*b, 40);
        let s = InserterHandle::new(String::from("fast-inserter"));
        assert_eq!(label_len(&s), 13, "&InserterHandle<String> should coerce to &str");
    }
}
