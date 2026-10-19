use std::f64::consts::PI;

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
        // TODO: return PI * r * r
        todo!()
    }

    fn name(&self) -> String {
        "circle".to_string()
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        // TODO: return w * h
        todo!()
    }

    fn name(&self) -> String {
        "rect".to_string()
    }
}

/// A circle with r = 1.0 and a 2.0 x 3.0 rect, in that order.
pub fn make_shapes() -> Vec<Box<dyn Shape>> {
    // TODO: build a Vec of boxed shapes: Circle then Rect
    Vec::new()
}

/// Sum of all areas.
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    // TODO: add up shape.area() for every shape
    todo!()
}

/// "<name> with area <one decimal>", e.g. "rect with area 6.0".
pub fn describe(shape: &dyn Shape) -> String {
    // TODO: format name and area (one decimal place: {:.1})
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn shapes_report_area() {
        assert!(close(Circle { r: 2.0 }.area(), 4.0 * PI), "circle area is PI * r * r");
        assert!(close(Rect { w: 2.0, h: 5.0 }.area(), 10.0), "rect area is w * h");
    }

    #[test]
    fn make_shapes_builds_mixed_vec() {
        let shapes = make_shapes();
        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0].name(), "circle");
        assert_eq!(shapes[1].name(), "rect");
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

    #[test]
    fn describe_takes_a_reference() {
        assert_eq!(describe(&Rect { w: 2.0, h: 3.0 }), "rect with area 6.0");
        let boxed: Box<dyn Shape> = Box::new(Circle { r: 1.0 });
        assert_eq!(describe(boxed.as_ref()), "circle with area 3.1");
    }
}
