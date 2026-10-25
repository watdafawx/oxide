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
        // TODO: return the label "circle with radius <radius>", e.g. "circle with radius 2"
        todo!()
    }
}

impl Describe for Square {
    fn describe(&self) -> String {
        // TODO: return the label "square with side <side>", e.g. "square with side 3"
        todo!()
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
