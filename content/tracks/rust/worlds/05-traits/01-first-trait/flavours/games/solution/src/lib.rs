pub trait Entity {
    fn describe(&self) -> String;
}

pub struct Zombie {
    pub name: String,
}

pub struct Creeper {
    pub fuse: u32,
}

impl Entity for Zombie {
    fn describe(&self) -> String {
        format!("{} the zombie", self.name)
    }
}

impl Entity for Creeper {
    fn describe(&self) -> String {
        format!("creeper (fuse {})", self.fuse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zombie_describes_itself() {
        let z = Zombie { name: "Steve".to_string() };
        assert_eq!(z.describe(), "Steve the zombie");
    }

    #[test]
    fn creeper_describes_itself() {
        assert_eq!(Creeper { fuse: 30 }.describe(), "creeper (fuse 30)");
    }
}
