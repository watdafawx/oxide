pub mod geometry {
    /// Private: children can see it through `super::PI`.
    const PI: f64 = 3.0;

    pub mod circle {
        /// Area is PI * r * r.
        pub fn area(r: f64) -> f64 {
            // TODO: area of the circle, using the parent module's private constant
            todo!()
        }
    }

    pub mod square {
        pub fn area(side: f64) -> f64 {
            // TODO: area of the square
            todo!()
        }
    }

    /// Circle of radius `r` plus square of side `side`.
    pub fn total_area(r: f64, side: f64) -> f64 {
        // TODO: add the circle's area to the square's, calling into the child modules
        todo!()
    }
}

/// Text such as "total area: 7".
pub fn describe(r: f64, side: f64) -> String {
    // TODO: bring `total_area` into scope with `use`,
    //       then format "total area: <total>"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area_uses_parent_constant() {
        assert_eq!(geometry::circle::area(2.0), 12.0, "PI is 3.0 here, so 3 * 2 * 2");
    }

    #[test]
    fn square_area_multiplies_sides() {
        assert_eq!(geometry::square::area(4.0), 16.0);
    }

    #[test]
    fn total_area_adds_both_children() {
        assert_eq!(geometry::total_area(1.0, 2.0), 7.0, "3 for the circle + 4 for the square");
    }

    #[test]
    fn describe_uses_an_imported_function() {
        assert_eq!(describe(1.0, 2.0), "total area: 7");
    }
}
