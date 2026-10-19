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
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f64) -> Vec3 {
        Vec3::new(self.x * k, self.y * k, self.z * k)
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Vec3) -> Option<Ordering> {
        self.len_sq().partial_cmp(&other.len_sq())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_plus_velocity() {
        let pos = Vec3::new(1.0, 2.0, 3.0);
        let velocity = Vec3::new(10.0, 20.0, 30.0);
        assert_eq!(pos + velocity, Vec3::new(11.0, 22.0, 33.0));
    }

    #[test]
    fn negated_vector_flips_components() {
        assert_eq!(-Vec3::new(3.0, -4.0, 0.0), Vec3::new(-3.0, 4.0, -0.0));
    }

    #[test]
    fn vector_scales_by_number() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.5) * 3.0, Vec3::new(3.0, -6.0, 1.5));
    }

    #[test]
    fn closer_vector_is_smaller() {
        let near = Vec3::new(1.0, 1.0, 1.0);
        let far = Vec3::new(0.0, 0.0, 5.0);
        assert!(near < far, "(1, 1, 1) is closer than (0, 0, 5)");
        assert!(far > near);
        assert!(Vec3::new(3.0, 4.0, 0.0) <= Vec3::new(0.0, 5.0, 0.0), "both have length 5");
    }
}
