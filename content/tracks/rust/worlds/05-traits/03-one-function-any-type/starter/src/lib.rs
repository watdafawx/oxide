pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub r: f64,
}

pub struct Square {
    pub side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.0 * self.r * self.r // close enough to pi for a sketch
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// All areas added together.
pub fn total_area<T: Shape>(shapes: &[T]) -> f64 {
    // TODO: add up area() of every shape
    todo!()
}

/// The shape with the largest area. `shapes` is never empty; ties go to the first.
pub fn biggest<T: Shape>(shapes: &[T]) -> &T {
    // TODO: keep a reference to the best shape so far and replace it when one has a larger area
    &shapes[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_area_works_for_any_shape() {
        let circles = [Circle { r: 1.0 }, Circle { r: 2.0 }];
        assert_eq!(total_area(&circles), 15.0, "3 + 12");
        let squares = [Square { side: 2.0 }, Square { side: 3.0 }];
        assert_eq!(total_area(&squares), 13.0, "4 + 9");
        assert_eq!(total_area::<Square>(&[]), 0.0);
    }

    #[test]
    fn biggest_picks_the_largest_area() {
        let squares = [Square { side: 1.0 }, Square { side: 4.0 }, Square { side: 2.0 }];
        assert_eq!(biggest(&squares).side, 4.0);
        let circles = [Circle { r: 3.0 }, Circle { r: 1.0 }];
        assert_eq!(biggest(&circles).r, 3.0);
    }
}
