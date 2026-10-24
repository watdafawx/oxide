use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// How far away something is, in blocks. Equal and ordered by the same number.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Distance(pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// The distance of this offset from the zombie.
    pub fn distance(&self) -> Distance {
        Distance((self.x * self.x + self.y * self.y + self.z * self.z).sqrt())
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Distance) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// The offset (player position minus zombie position) with the smaller distance. Ties go to `a`.
pub fn closer(a: Vec3, b: Vec3) -> Vec3 {
    if a.distance() <= b.distance() { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distances_compare_by_size() {
        let near = Vec3::new(1.0, 1.0, 0.0).distance();
        let far = Vec3::new(0.0, 0.0, 5.0).distance();
        assert!(near < far);
        assert!(far > near);
        let a = Vec3::new(3.0, 4.0, 0.0);
        let b = Vec3::new(5.0, 0.0, 0.0);
        assert!(a.distance() <= b.distance() && b.distance() <= a.distance(), "both are 5 blocks away");
        assert_eq!(a.distance(), b.distance(), "and == agrees: the distances are equal");
        assert_ne!(a, b, "the offsets themselves are still different");
        assert_eq!(Distance(f64::NAN).partial_cmp(&Distance(1.0)), None, "NaN has no order");
    }

    #[test]
    fn closer_picks_the_nearer_mob() {
        let near = Vec3::new(1.0, 1.0, 0.0);
        let far = Vec3::new(0.0, 0.0, 5.0);
        assert_eq!(closer(far, near), near);
        assert_eq!(closer(near, far), near);
        assert_eq!(closer(Vec3::new(3.0, 4.0, 0.0), Vec3::new(5.0, 0.0, 0.0)), Vec3::new(3.0, 4.0, 0.0), "ties go to a");
    }
}
