//! The macro lives in `describe_macro/`. This crate uses it and tests it.
pub use describe_macro::Describe;

#[cfg(test)]
mod tests {
    use super::Describe;

    #[derive(Describe)]
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Describe)]
    #[allow(dead_code)]
    enum Shape {
        Circle,
        Square,
    }

    #[test]
    fn struct_says_name_and_field_count() {
        let p = Point { x: 1, y: 2 };
        assert_eq!(p.describe(), "Point with 2 fields");
    }

    #[test]
    fn enum_says_enum() {
        assert_eq!(Shape::Circle.describe(), "Shape (enum)");
        assert_eq!(Shape::Square.describe(), "Shape (enum)");
    }
}
