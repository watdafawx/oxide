pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
}

pub struct Circle {
    pub r: f64,
}

pub struct Rect {
    pub w: f64,
    pub h: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
    fn name(&self) -> String {
        "circle".to_string()
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
    fn name(&self) -> String {
        "rect".to_string()
    }
}

/// A circle with r = 1.0, then a 2.0 x 3.0 rect.
pub fn make_shapes() -> Vec<Box<dyn Shape>> {
    vec![Box::new(Circle { r: 1.0 }), Box::new(Rect { w: 2.0, h: 3.0 })]
}

/// Sum of all areas.
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn make_shapes_mixes_types() {
        let shapes = make_shapes();
        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0].name(), "circle");
        assert_eq!(shapes[1].name(), "rect");
        assert!(close(shapes[1].area(), 6.0), "the rect is 2 by 3");
    }

    #[test]
    fn total_area_sums_boxed_shapes() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rect { w: 1.0, h: 2.0 }),
            Box::new(Rect { w: 3.0, h: 3.0 }),
            Box::new(Circle { r: 1.0 }),
        ];
        assert!(close(total_area(&shapes), 11.0 + PI));
        assert!(close(total_area(&[]), 0.0));
    }
}
