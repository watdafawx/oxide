use std::f64::consts::PI;

pub enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
    Square(f64),
}

impl Shape {
    /// Circle(r) -> PI * r * r, Rect -> w * h, Square(side) -> side * side
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
            Shape::Rect { w, h } => w * h,
            Shape::Square(side) => side * side,
        }
    }

    /// "circle", "rect" or "square"
    pub fn name(&self) -> &'static str {
        match self {
            Shape::Circle(_) => "circle",
            Shape::Rect { .. } => "rect",
            Shape::Square(_) => "square",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn circle_area_uses_pi() {
        let a = Shape::Circle(2.0).area();
        assert!(close(a, 4.0 * PI), "Circle(2.0) area should be 4*PI, got {a}");
    }

    #[test]
    fn rect_and_square_areas() {
        let r = Shape::Rect { w: 3.0, h: 4.0 }.area();
        let s = Shape::Square(5.0).area();
        assert!(close(r, 12.0), "Rect 3x4 should be 12, got {r}");
        assert!(close(s, 25.0), "Square(5) should be 25, got {s}");
    }

    #[test]
    fn names_describe_each_variant() {
        assert_eq!(Shape::Circle(1.0).name(), "circle");
        assert_eq!(Shape::Rect { w: 1.0, h: 2.0 }.name(), "rect");
        assert_eq!(Shape::Square(1.0).name(), "square");
    }
}
