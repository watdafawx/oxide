//! The macro lives in `derive_hello/`. This crate re-exports it and tests it.
pub use derive_hello::Hello;

#[cfg(test)]
mod tests {
    use super::Hello;

    #[derive(Hello)]
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Hello)]
    #[allow(dead_code)]
    enum Shape {
        Circle,
        Square,
    }

    #[derive(Hello)]
    #[allow(dead_code)]
    struct Pair<T: Clone> {
        a: T,
        b: T,
        c: T,
    }

    #[test]
    fn struct_reports_field_count() {
        let p = Point { x: 1, y: 2 };
        assert_eq!(p.hello(), "Point with 2 fields");
    }

    #[test]
    fn enum_says_enum() {
        assert_eq!(Shape::Circle.hello(), "Shape (enum)");
        assert_eq!(Shape::Square.hello(), "Shape (enum)");
    }

    #[test]
    fn generic_struct_works() {
        let p = Pair { a: 1u8, b: 2, c: 3 };
        assert_eq!(p.hello(), "Pair with 3 fields");
    }
}
