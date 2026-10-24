use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, k: f64) -> Vec2 {
        // TODO: a new Vec2 with both parts multiplied by k
        let _ = k;
        self
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        // TODO: the same scaling; here self is the number and v the vector
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_times_number() {
        assert_eq!(Vec2::new(1.0, -2.0) * 3.0, Vec2::new(3.0, -6.0));
        assert_eq!(Vec2::new(4.0, 2.0) * 0.5, Vec2::new(2.0, 1.0));
    }

    #[test]
    fn number_times_vector() {
        assert_eq!(2.5 * Vec2::new(2.0, 4.0), Vec2::new(5.0, 10.0));
        assert_eq!(3.0 * Vec2::new(1.0, -2.0), Vec2::new(1.0, -2.0) * 3.0, "both orders agree");
    }
}
