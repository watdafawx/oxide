use std::ops::{Add, Neg};

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

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        // TODO: a new Vec2 with the x parts added and the y parts added
        let _ = other;
        self
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        // TODO: a new Vec2 with the sign of x and y flipped
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vectors() {
        let point = Vec2::new(1.0, 2.0);
        let moved = point + Vec2::new(10.0, 20.0);
        assert_eq!(moved, Vec2::new(11.0, 22.0));
        assert_eq!(point, Vec2::new(1.0, 2.0), "Copy: point still works after +");
    }

    #[test]
    fn negate_vector() {
        assert_eq!(-Vec2::new(3.0, -4.0), Vec2::new(-3.0, 4.0));
    }
}
