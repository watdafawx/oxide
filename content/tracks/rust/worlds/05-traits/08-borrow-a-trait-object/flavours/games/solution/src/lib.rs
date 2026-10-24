pub trait Entity {
    fn health(&self) -> u32;
    fn name(&self) -> String;
}

pub struct Zombie {
    pub hp: u32,
}

pub struct Creeper {
    pub hp: u32,
    pub fuse: u32,
}

impl Entity for Zombie {
    fn health(&self) -> u32 {
        self.hp
    }
    fn name(&self) -> String {
        "zombie".to_string()
    }
}

impl Entity for Creeper {
    fn health(&self) -> u32 {
        self.hp
    }
    fn name(&self) -> String {
        "creeper".to_string()
    }
}

/// "<name> with <health> hp", e.g. "zombie with 20 hp".
pub fn describe(mob: &dyn Entity) -> String {
    format!("{} with {} hp", mob.name(), mob.health())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_takes_a_loose_mob() {
        assert_eq!(describe(&Zombie { hp: 20 }), "zombie with 20 hp");
    }

    #[test]
    fn describe_takes_mobs_from_the_world() {
        let world: Vec<Box<dyn Entity>> = vec![Box::new(Creeper { hp: 15, fuse: 30 }), Box::new(Zombie { hp: 3 })];
        let lines: Vec<String> = world.iter().map(|m| describe(m.as_ref())).collect();
        assert_eq!(lines, vec!["creeper with 15 hp", "zombie with 3 hp"]);
    }
}
