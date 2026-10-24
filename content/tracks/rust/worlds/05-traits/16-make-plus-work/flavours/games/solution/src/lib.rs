use std::ops::{Add, Neg};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_plus_velocity() {
        let pos = Vec3::new(10.0, 64.0, -3.0);
        let next = pos + Vec3::new(0.5, -1.0, 2.0);
        assert_eq!(next, Vec3::new(10.5, 63.0, -1.0));
        assert_eq!(pos, Vec3::new(10.0, 64.0, -3.0), "Copy: pos still works after +");
    }

    #[test]
    fn negated_vector_flips_parts() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0), "knockback points the other way");
    }
}
