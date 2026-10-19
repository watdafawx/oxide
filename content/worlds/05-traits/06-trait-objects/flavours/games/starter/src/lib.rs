pub trait Entity {
    fn health(&self) -> u32;
    fn name(&self) -> String;
}

pub struct Zombie {
    pub hp: u32,
}

pub struct Creeper {
    pub hp: u32,
}

impl Entity for Zombie {
    fn health(&self) -> u32 {
        // TODO: return the hp field
        todo!()
    }

    fn name(&self) -> String {
        "zombie".to_string()
    }
}

impl Entity for Creeper {
    fn health(&self) -> u32 {
        // TODO: return the hp field
        todo!()
    }

    fn name(&self) -> String {
        "creeper".to_string()
    }
}

/// A zombie with 20 hp and a creeper with 15 hp, in that order.
pub fn spawn_wave() -> Vec<Box<dyn Entity>> {
    // TODO: build a Vec of boxed mobs: Zombie then Creeper
    Vec::new()
}

/// Health of the whole world added together.
pub fn total_health(world: &[Box<dyn Entity>]) -> u32 {
    // TODO: add up mob.health() for every mob
    todo!()
}

/// "<name> with <hp> hp", e.g. "zombie with 20 hp".
pub fn describe(mob: &dyn Entity) -> String {
    // TODO: format the name and the health
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shapes_report_area() {
        assert_eq!(Zombie { hp: 7 }.health(), 7, "a zombie reports its hp");
        assert_eq!(Creeper { hp: 3 }.health(), 3, "a creeper reports its hp");
    }

    #[test]
    fn make_shapes_builds_mixed_vec() {
        let wave = spawn_wave();
        assert_eq!(wave.len(), 2);
        assert_eq!(wave[0].name(), "zombie");
        assert_eq!(wave[1].name(), "creeper");
    }

    #[test]
    fn total_area_sums_boxed_shapes() {
        let world: Vec<Box<dyn Entity>> = vec![
            Box::new(Zombie { hp: 20 }),
            Box::new(Zombie { hp: 5 }),
            Box::new(Creeper { hp: 15 }),
        ];
        assert_eq!(total_health(&world), 40);
        assert_eq!(total_health(&[]), 0);
        assert_eq!(total_health(&spawn_wave()), 35);
    }

    #[test]
    fn describe_takes_a_reference() {
        assert_eq!(describe(&Zombie { hp: 20 }), "zombie with 20 hp");
        let boxed: Box<dyn Entity> = Box::new(Creeper { hp: 15 });
        assert_eq!(describe(boxed.as_ref()), "creeper with 15 hp");
    }
}
