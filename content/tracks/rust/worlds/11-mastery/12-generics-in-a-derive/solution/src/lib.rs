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

    /// Generic: holds three values of any type that can be cloned.
    #[derive(Describe)]
    #[allow(dead_code)]
    struct Triple<T: Clone> {
        a: T,
        b: T,
        c: T,
    }

    #[test]
    fn generic_struct_gets_a_correct_impl() {
        let t = Triple { a: 1u8, b: 2, c: 3 };
        assert_eq!(t.describe(), "Triple with 3 fields");
        let words = Triple { a: "x", b: "y", c: "z" };
        assert_eq!(words.describe(), "Triple with 3 fields");
    }

    #[test]
    fn plain_struct_still_works() {
        assert_eq!(Point { x: 1, y: 2 }.describe(), "Point with 2 fields");
    }
}
