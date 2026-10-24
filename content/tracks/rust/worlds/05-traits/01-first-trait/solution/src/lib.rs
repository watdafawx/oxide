pub trait Describe {
    fn describe(&self) -> String;
}

pub struct Circle {
    pub radius: u32,
}

pub struct Square {
    pub side: u32,
}

impl Describe for Circle {
    fn describe(&self) -> String {
        format!("circle with radius {}", self.radius)
    }
}

impl Describe for Square {
    fn describe(&self) -> String {
        format!("square with side {}", self.side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_describes_itself() {
        assert_eq!(Circle { radius: 2 }.describe(), "circle with radius 2");
    }

    #[test]
    fn square_describes_itself() {
        assert_eq!(Square { side: 3 }.describe(), "square with side 3");
    }
}
