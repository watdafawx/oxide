use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub trait Hitbox {
    fn area(&self) -> f64;
    /// Smallest Aabb around the hitbox.
    fn bounds(&self) -> Aabb;
    fn name(&self) -> String;
    /// Is the point inside the hitbox (edges included)?
    fn contains(&self, x: f64, y: f64) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Round hitbox seen from above: centre (cx, cy) and radius r.
pub struct Cylinder {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

/// Right-angled wedge: right angle at (x, y), legs go right (`base`) and down (`height`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wedge {
    pub x: f64,
    pub y: f64,
    pub base: f64,
    pub height: f64,
}

/// Any hitbox, scaled by `factor` around its top-left corner.
pub struct Scaled<H: Hitbox> {
    pub inner: H,
    pub factor: f64,
}

pub struct Canvas {
    pub w: usize,
    pub h: usize,
}

impl Hitbox for Aabb {
    fn area(&self) -> f64 {
        // TODO: width times height
        todo!()
    }

    fn bounds(&self) -> Aabb {
        // TODO: an Aabb is its own bounds
        *self
    }

    fn name(&self) -> String {
        // TODO: return "box"
        String::new()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        // TODO: true when x and y are inside the rect, edges included
        false
    }
}

impl Hitbox for Cylinder {
    fn area(&self) -> f64 {
        // TODO: PI * r * r
        0.0
    }

    fn bounds(&self) -> Aabb {
        // TODO: the square that touches the cylinder on all four sides
        Aabb { x: 0.0, y: 0.0, w: 0.0, h: 0.0 }
    }

    fn name(&self) -> String {
        // TODO: return "cylinder"
        String::new()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        // TODO: distance squared from the centre <= r squared
        false
    }
}

impl Hitbox for Wedge {
    fn area(&self) -> f64 {
        // TODO: base * height / 2
        0.0
    }

    fn bounds(&self) -> Aabb {
        // TODO: an Aabb from (x, y) that is base wide and height tall
        Aabb { x: 0.0, y: 0.0, w: 0.0, h: 0.0 }
    }

    fn name(&self) -> String {
        // TODO: return "wedge"
        String::new()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        // TODO: dx, dy >= 0 and dx / base + dy / height <= 1
        false
    }
}

impl<H: Hitbox> Hitbox for Scaled<H> {
    fn area(&self) -> f64 {
        // TODO: inner area times factor squared
        0.0
    }

    fn bounds(&self) -> Aabb {
        // TODO: keep x and y of the inner bounds, scale w and h
        Aabb { x: 0.0, y: 0.0, w: 0.0, h: 0.0 }
    }

    fn name(&self) -> String {
        // TODO: return "scaled <inner name>", e.g. "scaled box"
        String::new()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        // TODO: map (x, y) back into the unscaled hitbox around its top-left corner, then ask the inner hitbox
        false
    }
}

impl Canvas {
    /// One char per cell: '#' if any hitbox contains the cell centre, else '.'.
    /// Rows joined with '\n', no trailing newline.
    pub fn render(&self, hitboxes: &[Box<dyn Hitbox>]) -> String {
        // TODO: loop over rows and columns, test (col + 0.5, row + 0.5), join rows with "\n"
        String::new()
    }
}

/// Sum of all areas.
pub fn total_area<T: Hitbox>(hitboxes: &[T]) -> f64 {
    // TODO: add up the areas
    0.0
}

/// The hitbox with the biggest area, or None for an empty slice.
pub fn biggest<T: Hitbox>(hitboxes: &[T]) -> Option<&T> {
    // TODO: find the item with the biggest area (f64 has total_cmp)
    None
}

/// Sort in place, smallest area first.
pub fn sort_by_area<T: Hitbox>(hitboxes: &mut [T]) {
    // TODO: sort_by comparing areas with total_cmp
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn aabb(x: f64, y: f64, w: f64, h: f64) -> Aabb {
        Aabb { x, y, w, h }
    }

    #[test]
    fn rect_area_and_name() {
        let r = aabb(0.0, 0.0, 2.0, 3.0);
        assert!(close(r.area(), 6.0), "area is w * h");
        assert_eq!(r.name(), "box");
    }

    #[test]
    fn rect_contains_is_inclusive() {
        let r = aabb(1.0, 1.0, 2.0, 1.0);
        assert!(r.contains(2.0, 1.5), "inside");
        assert!(r.contains(1.0, 1.0), "top-left corner counts");
        assert!(r.contains(3.0, 2.0), "bottom-right corner counts");
        assert!(!r.contains(3.1, 1.5), "just right of the box");
        assert!(!r.contains(2.0, 0.9), "just above the box");
        assert_eq!(r.bounds(), r, "an Aabb is its own bounds");
    }

    #[test]
    fn circle_is_a_shape() {
        let c = Cylinder { cx: 2.5, cy: 2.5, r: 2.0 };
        assert!(close(c.area(), 4.0 * PI));
        assert_eq!(c.name(), "cylinder");
        assert_eq!(c.bounds(), aabb(0.5, 0.5, 4.0, 4.0));
        assert!(c.contains(2.5, 2.5), "centre");
        assert!(c.contains(4.5, 2.5), "on the edge");
        assert!(!c.contains(4.5, 4.5), "corner of the bounds is outside");
    }

    #[test]
    fn triangle_is_a_shape() {
        let t = Wedge { x: 0.0, y: 0.0, base: 4.0, height: 4.0 };
        assert!(close(t.area(), 8.0));
        assert_eq!(t.name(), "wedge");
        assert_eq!(t.bounds(), aabb(0.0, 0.0, 4.0, 4.0));
        assert!(t.contains(0.5, 0.5), "near the right angle");
        assert!(t.contains(3.0, 0.5), "along the base");
        assert!(!t.contains(3.0, 3.0), "past the hypotenuse");
        assert!(!t.contains(-1.0, 1.0), "left of the wedge");
    }

    #[test]
    fn canvas_renders_shapes() {
        let canvas = Canvas { w: 6, h: 3 };
        let one: Vec<Box<dyn Hitbox>> = vec![Box::new(aabb(1.0, 1.0, 2.0, 1.0))];
        assert_eq!(canvas.render(&one), "......\n.##...\n......");

        let canvas = Canvas { w: 5, h: 5 };
        let cylinder: Vec<Box<dyn Hitbox>> = vec![Box::new(Cylinder { cx: 2.5, cy: 2.5, r: 2.0 })];
        assert_eq!(canvas.render(&cylinder), "..#..\n.###.\n#####\n.###.\n..#..");

        let mixed: Vec<Box<dyn Hitbox>> = vec![
            Box::new(aabb(0.0, 0.0, 1.0, 1.0)),
            Box::new(Wedge { x: 2.0, y: 1.0, base: 3.0, height: 2.0 }),
        ];
        let canvas = Canvas { w: 5, h: 3 };
        assert_eq!(canvas.render(&mixed), "#....\n..##.\n..#..");
        assert_eq!(Canvas { w: 2, h: 1 }.render(&[]), "..", "no hitboxes, nothing drawn");
    }

    #[test]
    fn totals_and_largest() {
        let boxes = [aabb(0.0, 0.0, 1.0, 2.0), aabb(0.0, 0.0, 3.0, 3.0), aabb(0.0, 0.0, 2.0, 2.0)];
        assert!(close(total_area(&boxes), 15.0));
        assert_eq!(biggest(&boxes), Some(&boxes[1]));
        let cylinders = [Cylinder { cx: 0.0, cy: 0.0, r: 1.0 }];
        assert!(close(total_area(&cylinders), PI));
        let none: [Aabb; 0] = [];
        assert_eq!(biggest(&none), None);
    }

    #[test]
    fn sort_by_area_orders_shapes() {
        let mut boxes = vec![aabb(0.0, 0.0, 3.0, 3.0), aabb(0.0, 0.0, 1.0, 1.0), aabb(0.0, 0.0, 2.0, 2.0)];
        sort_by_area(&mut boxes);
        let areas: Vec<f64> = boxes.iter().map(|r| r.area()).collect();
        assert_eq!(areas, vec![1.0, 4.0, 9.0]);
    }

    #[test]
    fn scaled_wraps_any_shape() {
        let big = Scaled { inner: aabb(1.0, 1.0, 2.0, 1.0), factor: 2.0 };
        assert!(close(big.area(), 8.0), "area grows with factor squared");
        assert_eq!(big.name(), "scaled box");
        assert_eq!(big.bounds(), aabb(1.0, 1.0, 4.0, 2.0));
        assert!(big.contains(4.5, 2.5), "inside the grown box");
        assert!(!big.contains(5.5, 2.0), "outside the grown box");

        let ring = Scaled { inner: Cylinder { cx: 1.0, cy: 1.0, r: 1.0 }, factor: 2.0 };
        assert_eq!(ring.name(), "scaled cylinder");
        assert!(close(ring.area(), 4.0 * PI));

        let canvas = Canvas { w: 4, h: 2 };
        let hitboxes: Vec<Box<dyn Hitbox>> = vec![Box::new(Scaled { inner: aabb(0.0, 0.0, 1.0, 1.0), factor: 2.0 })];
        assert_eq!(canvas.render(&hitboxes), "##..\n##..", "a Scaled works as a Box<dyn Hitbox>");
    }
}
