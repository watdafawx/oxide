#[derive(Debug, PartialEq, Clone)]
pub struct Label {
    pub text: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Two independent copies of the label.
pub fn two_labels(label: &Label) -> Vec<Label> {
    vec![label.clone(), label.clone()]
}

/// The same point twice.
pub fn mirror(p: Point) -> (Point, Point) {
    (p, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_labels_clones() {
        let label = Label { text: "door".to_string() };
        let copies = two_labels(&label);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0], label);
        assert_eq!(copies[1], label);
    }

    #[test]
    fn mirror_returns_point_twice() {
        assert_eq!(mirror(Point { x: 3, y: 4 }), (Point { x: 3, y: 4 }, Point { x: 3, y: 4 }));
    }
}
