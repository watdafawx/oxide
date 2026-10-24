use std::ops::Mul;

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

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f64) -> Vec3 {
        // TODO: a new Vec3 with all three parts multiplied by k
        let _ = k;
        self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        // TODO: the same scaling; here self is the number and v the vector
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn velocity_times_number() {
        assert_eq!(Vec3::new(1.0, -2.0, 4.0) * 0.5, Vec3::new(0.5, -1.0, 2.0), "half a second of movement");
    }

    #[test]
    fn number_times_velocity() {
        assert_eq!(2.0 * Vec3::new(1.0, 0.0, -3.0), Vec3::new(2.0, 0.0, -6.0));
        assert_eq!(0.5 * Vec3::new(1.0, -2.0, 4.0), Vec3::new(1.0, -2.0, 4.0) * 0.5, "both orders agree");
    }
}
