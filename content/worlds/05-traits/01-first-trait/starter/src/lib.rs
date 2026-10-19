pub trait Describe {
    fn describe(&self) -> String;
}

pub struct Dog {
    pub name: String,
}

pub struct Robot {
    pub id: u32,
}

impl Describe for Dog {
    fn describe(&self) -> String {
        // TODO: return "<name> the dog", e.g. "Rex the dog"
        todo!()
    }
}

impl Describe for Robot {
    fn describe(&self) -> String {
        // TODO: return "robot #<id>", e.g. "robot #7"
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dog_describes_itself() {
        let d = Dog { name: "Rex".to_string() };
        assert_eq!(d.describe(), "Rex the dog");
    }

    #[test]
    fn robot_describes_itself() {
        let r = Robot { id: 7 };
        assert_eq!(r.describe(), "robot #7");
    }
}
