use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// Length squared: cheaper than the length and orders the same way.
    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        // TODO: add the x, y and z parts
        self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        // TODO: flip the sign of x, y and z
        self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f64) -> Vec3 {
        // TODO: multiply x, y and z by k
        self
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Vec3) -> Option<Ordering> {
        // TODO: compare len_sq() of both vectors
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vectors() {
        let pos = Vec3::new(1.0, 2.0, 3.0);
        let velocity = Vec3::new(10.0, 20.0, 30.0);
        assert_eq!(pos + velocity, Vec3::new(11.0, 22.0, 33.0));
    }

    #[test]
    fn negate_vector() {
        assert_eq!(-Vec3::new(3.0, -4.0, 0.0), Vec3::new(-3.0, 4.0, -0.0));
    }

    #[test]
    fn scale_by_number() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.5) * 3.0, Vec3::new(3.0, -6.0, 1.5));
    }

    #[test]
    fn vectors_compare_by_length() {
        let near = Vec3::new(1.0, 1.0, 1.0);
        let far = Vec3::new(0.0, 0.0, 5.0);
        assert!(near < far, "(1, 1, 1) is closer than (0, 0, 5)");
        assert!(far > near);
        assert!(Vec3::new(3.0, 4.0, 0.0) <= Vec3::new(0.0, 5.0, 0.0), "both have length 5");
    }
}
