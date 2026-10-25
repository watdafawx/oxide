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

/// A zombie with 20 hp, then a creeper with 15 hp and fuse 30.
pub fn spawn_wave() -> Vec<Box<dyn Entity>> {
    // TODO: build a Vec of boxed mobs: the zombie, then the creeper
    Vec::new()
}

/// Health of every mob in the world, added up.
pub fn total_health(world: &[Box<dyn Entity>]) -> u32 {
    // TODO: add up health() for every mob
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_wave_mixes_mob_types() {
        let wave = spawn_wave();
        assert_eq!(wave.len(), 2);
        assert_eq!(wave[0].name(), "zombie");
        assert_eq!(wave[1].name(), "creeper");
        assert_eq!(wave[1].health(), 15);
    }

    #[test]
    fn total_health_sums_boxed_mobs() {
        let world: Vec<Box<dyn Entity>> = vec![
            Box::new(Zombie { hp: 20 }),
            Box::new(Creeper { hp: 15, fuse: 30 }),
            Box::new(Zombie { hp: 5 }),
        ];
        assert_eq!(total_health(&world), 40);
        assert_eq!(total_health(&[]), 0);
    }
}
