use std::ops::Deref;

/// A handle to a machine. It owns the machine and acts like a reference to it.
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

/// Takes a plain &str, so passing a &InserterHandle<String> relies on deref coercion.
pub fn label_len(label: &str) -> usize {
    label.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_reads_the_machine_inside() {
        let h = InserterHandle::new(40);
        assert_eq!(*h, 40);
    }

    #[test]
    fn handle_of_string_works_as_str() {
        let label = InserterHandle::new(String::from("fast-inserter"));
        assert_eq!(label_len(&label), 13, "&InserterHandle<String> should coerce to &str");
    }
}
