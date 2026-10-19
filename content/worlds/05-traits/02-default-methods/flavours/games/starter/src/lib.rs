pub trait Entity {
    fn name(&self) -> String;

    fn is_hostile(&self) -> bool {
        // TODO: mobs are peaceful unless they say otherwise: return false
        todo!()
    }

    fn announce(&self) -> String {
        // TODO: return "A wild <name> appears!" using self.name()
        todo!()
    }

    fn alert(&self) -> String {
        // TODO: return self.announce() in uppercase
        todo!()
    }
}

pub struct Cow;

impl Entity for Cow {
    fn name(&self) -> String {
        "Cow".to_string()
    }
}

pub struct Zombie;

impl Entity for Zombie {
    fn name(&self) -> String {
        "Zombie".to_string()
    }

    fn is_hostile(&self) -> bool {
        // TODO: zombies are hostile: return true
        todo!()
    }

    fn announce(&self) -> String {
        // TODO: return "<name> groans at you!"
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_greet_uses_name() {
        assert_eq!(Cow.announce(), "A wild Cow appears!");
        assert!(!Cow.is_hostile(), "mobs are peaceful by default");
    }

    #[test]
    fn shout_builds_on_greet() {
        assert_eq!(Cow.alert(), "A WILD COW APPEARS!");
        assert_eq!(
            Zombie.alert(),
            "ZOMBIE GROANS AT YOU!",
            "alert must call announce(), so overrides show through"
        );
    }

    #[test]
    fn parrot_overrides_greet() {
        assert_eq!(Zombie.announce(), "Zombie groans at you!");
        assert!(Zombie.is_hostile(), "Zombie overrides is_hostile");
    }
}
