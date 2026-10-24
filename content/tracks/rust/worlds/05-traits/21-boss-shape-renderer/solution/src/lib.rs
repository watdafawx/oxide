use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub trait Shape {
    fn area(&self) -> f64;
    /// Smallest Rect around the shape.
    fn bounds(&self) -> Rect;
    fn name(&self) -> String;
    /// Is the point inside the shape (edges included)?
    fn contains(&self, x: f64, y: f64) -> bool;
}

/// Centre (cx, cy), radius r. Area: PI * r * r. Bounds: the square from (cx - r, cy - r), 2r on each side.
/// Inside when dx * dx + dy * dy <= r * r, with dx = x - cx and dy = y - cy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

/// Right triangle: right angle at (x, y), legs go right (`base`) and down (`height`).
/// Area: base * height / 2. Bounds: the Rect from (x, y), base wide and height tall.
/// Inside when dx >= 0, dy >= 0 and dx / base + dy / height <= 1, with dx = px - x and dy = py - y.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub x: f64,
    pub y: f64,
    pub base: f64,
    pub height: f64,
}

/// Any shape, scaled by `factor` around its top-left corner. Area grows by factor * factor.
/// To test a point, map it back into the inner shape: with b = inner.bounds(),
/// ask inner.contains(b.x + (x - b.x) / factor, b.y + (y - b.y) / factor).
pub struct Scaled<S: Shape> {
    pub inner: S,
    pub factor: f64,
}

/// `w` columns and `h` rows. Cell (col, row) has its centre at (col as f64 + 0.5, row as f64 + 0.5).
pub struct Canvas {
    pub w: usize,
    pub h: usize,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }

    fn bounds(&self) -> Rect {
        *self
    }

    fn name(&self) -> String {
        "rect".to_string()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }

    fn bounds(&self) -> Rect {
        Rect {
            x: self.cx - self.r,
            y: self.cy - self.r,
            w: 2.0 * self.r,
            h: 2.0 * self.r,
        }
    }

    fn name(&self) -> String {
        "circle".to_string()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        let dx = x - self.cx;
        let dy = y - self.cy;
        dx * dx + dy * dy <= self.r * self.r
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }

    fn bounds(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.base,
            h: self.height,
        }
    }

    fn name(&self) -> String {
        "triangle".to_string()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;
        dx >= 0.0 && dy >= 0.0 && dx / self.base + dy / self.height <= 1.0
    }
}

impl<S: Shape> Shape for Scaled<S> {
    fn area(&self) -> f64 {
        self.inner.area() * self.factor * self.factor
    }

    fn bounds(&self) -> Rect {
        let b = self.inner.bounds();
        Rect {
            x: b.x,
            y: b.y,
            w: b.w * self.factor,
            h: b.h * self.factor,
        }
    }

    fn name(&self) -> String {
        format!("scaled {}", self.inner.name())
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        let b = self.inner.bounds();
        self.inner.contains(
            b.x + (x - b.x) / self.factor,
            b.y + (y - b.y) / self.factor,
        )
    }
}

impl Canvas {
    /// One char per cell: '#' if any shape contains the cell centre, else '.'.
    /// Rows joined with '\n', no trailing newline.
    pub fn render(&self, shapes: &[Box<dyn Shape>]) -> String {
        let mut rows = Vec::with_capacity(self.h);
        for row in 0..self.h {
            let line: String = (0..self.w)
                .map(|col| {
                    let (px, py) = (col as f64 + 0.5, row as f64 + 0.5);
                    if shapes.iter().any(|s| s.contains(px, py)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();
            rows.push(line);
        }
        rows.join("\n")
    }
}

/// Sum of all areas.
pub fn total_area<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

/// The shape with the biggest area, or None for an empty slice.
pub fn largest<T: Shape>(shapes: &[T]) -> Option<&T> {
    shapes.iter().max_by(|a, b| a.area().total_cmp(&b.area()))
}

/// Sort in place, smallest area first.
pub fn sort_by_area<T: Shape>(shapes: &mut [T]) {
    shapes.sort_by(|a, b| a.area().total_cmp(&b.area()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn rect(x: f64, y: f64, w: f64, h: f64) -> Rect {
        Rect { x, y, w, h }
    }

    #[test]
    fn rect_area_and_name() {
        let r = rect(0.0, 0.0, 2.0, 3.0);
        assert!(close(r.area(), 6.0), "area is w * h");
        assert_eq!(r.name(), "rect");
    }

    #[test]
    fn rect_contains_is_inclusive() {
        let r = rect(1.0, 1.0, 2.0, 1.0);
        assert!(r.contains(2.0, 1.5), "inside");
        assert!(r.contains(1.0, 1.0), "top-left corner counts");
        assert!(r.contains(3.0, 2.0), "bottom-right corner counts");
        assert!(!r.contains(3.1, 1.5), "just right of the rect");
        assert!(!r.contains(2.0, 0.9), "just above the rect");
        assert_eq!(r.bounds(), r, "a rect is its own bounds");
    }

    #[test]
    fn circle_is_a_shape() {
        let c = Circle { cx: 2.5, cy: 2.5, r: 2.0 };
        assert!(close(c.area(), 4.0 * PI));
        assert_eq!(c.name(), "circle");
        assert_eq!(c.bounds(), rect(0.5, 0.5, 4.0, 4.0));
        assert!(c.contains(2.5, 2.5), "centre");
        assert!(c.contains(4.5, 2.5), "on the edge");
        assert!(!c.contains(4.5, 4.5), "corner of the bounds is outside");
    }

    #[test]
    fn triangle_is_a_shape() {
        let t = Triangle { x: 0.0, y: 0.0, base: 4.0, height: 4.0 };
        assert!(close(t.area(), 8.0));
        assert_eq!(t.name(), "triangle");
        assert_eq!(t.bounds(), rect(0.0, 0.0, 4.0, 4.0));
        assert!(t.contains(0.5, 0.5), "near the right angle");
        assert!(t.contains(3.0, 0.5), "along the base");
        assert!(!t.contains(3.0, 3.0), "past the hypotenuse");
        assert!(!t.contains(-1.0, 1.0), "left of the triangle");
    }

    #[test]
    fn canvas_renders_shapes() {
        let canvas = Canvas { w: 6, h: 3 };
        let one: Vec<Box<dyn Shape>> = vec![Box::new(rect(1.0, 1.0, 2.0, 1.0))];
        assert_eq!(canvas.render(&one), "......\n.##...\n......");

        let canvas = Canvas { w: 5, h: 5 };
        let circle: Vec<Box<dyn Shape>> = vec![Box::new(Circle { cx: 2.5, cy: 2.5, r: 2.0 })];
        assert_eq!(canvas.render(&circle), "..#..\n.###.\n#####\n.###.\n..#..");

        let mixed: Vec<Box<dyn Shape>> = vec![
            Box::new(rect(0.0, 0.0, 1.0, 1.0)),
            Box::new(Triangle { x: 2.0, y: 1.0, base: 3.0, height: 2.0 }),
        ];
        let canvas = Canvas { w: 5, h: 3 };
        assert_eq!(canvas.render(&mixed), "#....\n..##.\n..#..");
        assert_eq!(Canvas { w: 2, h: 1 }.render(&[]), "..", "no shapes, nothing drawn");
    }

    #[test]
    fn totals_and_largest() {
        let rects = [rect(0.0, 0.0, 1.0, 2.0), rect(0.0, 0.0, 3.0, 3.0), rect(0.0, 0.0, 2.0, 2.0)];
        assert!(close(total_area(&rects), 15.0));
        assert_eq!(largest(&rects), Some(&rects[1]));
        let circles = [Circle { cx: 0.0, cy: 0.0, r: 1.0 }];
        assert!(close(total_area(&circles), PI));
        let none: [Rect; 0] = [];
        assert_eq!(largest(&none), None);
    }

    #[test]
    fn sort_by_area_orders_shapes() {
        let mut rects = vec![rect(0.0, 0.0, 3.0, 3.0), rect(0.0, 0.0, 1.0, 1.0), rect(0.0, 0.0, 2.0, 2.0)];
        sort_by_area(&mut rects);
        let areas: Vec<f64> = rects.iter().map(|r| r.area()).collect();
        assert_eq!(areas, vec![1.0, 4.0, 9.0]);
    }

    #[test]
    fn scaled_wraps_any_shape() {
        let big = Scaled { inner: rect(1.0, 1.0, 2.0, 1.0), factor: 2.0 };
        assert!(close(big.area(), 8.0), "area grows with factor squared");
        assert_eq!(big.name(), "scaled rect");
        assert_eq!(big.bounds(), rect(1.0, 1.0, 4.0, 2.0));
        assert!(big.contains(4.5, 2.5), "inside the grown rect");
        assert!(!big.contains(5.5, 2.0), "outside the grown rect");

        let ring = Scaled { inner: Circle { cx: 1.0, cy: 1.0, r: 1.0 }, factor: 2.0 };
        assert_eq!(ring.name(), "scaled circle");
        assert!(close(ring.area(), 4.0 * PI));

        let canvas = Canvas { w: 4, h: 2 };
        let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Scaled { inner: rect(0.0, 0.0, 1.0, 1.0), factor: 2.0 })];
        assert_eq!(canvas.render(&shapes), "##..\n##..", "a Scaled works as a Box<dyn Shape>");
    }
}
