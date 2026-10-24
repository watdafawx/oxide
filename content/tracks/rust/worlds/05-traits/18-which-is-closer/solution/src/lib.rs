use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

/// How long a vector is. Equal and ordered by the same number.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length(pub f64);

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn length(&self) -> Length {
        Length((self.x * self.x + self.y * self.y).sqrt())
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Length) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// The vector with the smaller length. Ties go to `a`.
pub fn closer(a: Vec2, b: Vec2) -> Vec2 {
    if a.length() <= b.length() { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths_compare_by_size() {
        let short = Vec2::new(1.0, 1.0).length();
        let long = Vec2::new(0.0, 5.0).length();
        assert!(short < long, "(1, 1) is shorter than (0, 5)");
        assert!(long > short);
        let a = Vec2::new(3.0, 4.0);
        let b = Vec2::new(5.0, 0.0);
        assert!(a.length() <= b.length() && b.length() <= a.length(), "both are 5 long");
        assert_eq!(a.length(), b.length(), "and == agrees: the lengths are equal");
        assert_ne!(a, b, "the vectors themselves are still different");
        assert_eq!(Length(f64::NAN).partial_cmp(&Length(1.0)), None, "NaN has no order");
    }

    #[test]
    fn closer_picks_the_shorter_vector() {
        assert_eq!(closer(Vec2::new(0.0, 5.0), Vec2::new(1.0, 1.0)), Vec2::new(1.0, 1.0));
        assert_eq!(closer(Vec2::new(1.0, 1.0), Vec2::new(0.0, 5.0)), Vec2::new(1.0, 1.0));
        assert_eq!(closer(Vec2::new(3.0, 4.0), Vec2::new(5.0, 0.0)), Vec2::new(3.0, 4.0), "ties go to a");
    }
}
