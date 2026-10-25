#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Label {
    pub text: String,
}

/// The same point twice.
pub fn mirror(p: Point) -> (Point, Point) {
    (p, p)
}

/// Two labels with the same text, each owning its own.
pub fn two_labels(label: Label) -> (Label, Label) {
    (label.clone(), label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_returns_the_point_twice() {
        let p = Point { x: 3, y: 4 };
        assert_eq!(mirror(p), (Point { x: 3, y: 4 }, Point { x: 3, y: 4 }));
    }

    #[test]
    fn two_labels_are_equal_copies() {
        let (a, b) = two_labels(Label { text: "door".to_string() });
        assert_eq!(a, b);
        assert_eq!(a.text, "door");
        assert_ne!(a.text.as_ptr(), b.text.as_ptr(), "each label owns its own letters");
    }
}
