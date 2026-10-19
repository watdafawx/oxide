use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    /// Length squared: cheaper than the length and orders the same way.
    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, k: f64) -> Vec2 {
        Vec2 {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Vec2) -> Option<Ordering> {
        self.len_sq().partial_cmp(&other.len_sq())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vectors() {
        let sum = Vec2::new(1.0, 2.0) + Vec2::new(10.0, 20.0);
        assert_eq!(sum, Vec2::new(11.0, 22.0));
    }

    #[test]
    fn negate_vector() {
        assert_eq!(-Vec2::new(3.0, -4.0), Vec2::new(-3.0, 4.0));
    }

    #[test]
    fn scale_by_number() {
        assert_eq!(Vec2::new(1.0, -2.0) * 3.0, Vec2::new(3.0, -6.0));
    }

    #[test]
    fn vectors_compare_by_length() {
        let short = Vec2::new(1.0, 1.0);
        let long = Vec2::new(0.0, 5.0);
        assert!(short < long, "(1, 1) is shorter than (0, 5)");
        assert!(long > short);
        assert!(Vec2::new(3.0, 4.0) <= Vec2::new(5.0, 0.0), "both have length 5");
    }
}
