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
        // TODO: return the label "<name> the zombie", e.g. "Steve the zombie"
        todo!()
    }
}

impl Entity for Creeper {
    fn describe(&self) -> String {
        // TODO: return the label "creeper (fuse <fuse>)", e.g. "creeper (fuse 30)"
        todo!()
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
