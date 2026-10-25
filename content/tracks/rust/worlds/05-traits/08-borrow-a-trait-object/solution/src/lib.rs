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

/// "<name> with area <one decimal>", e.g. "rect with area 6.0".
pub fn describe(shape: &dyn Shape) -> String {
    format!("{} with area {:.1}", shape.name(), shape.area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_takes_a_loose_shape() {
        assert_eq!(describe(&Rect { w: 2.0, h: 3.0 }), "rect with area 6.0");
    }

    #[test]
    fn describe_takes_shapes_from_the_list() {
        let canvas: Vec<Box<dyn Shape>> = vec![Box::new(Circle { r: 1.0 }), Box::new(Rect { w: 1.0, h: 1.0 })];
        let lines: Vec<String> = canvas.iter().map(|s| describe(s.as_ref())).collect();
        assert_eq!(lines, vec!["circle with area 3.1", "rect with area 1.0"]);
    }
}
