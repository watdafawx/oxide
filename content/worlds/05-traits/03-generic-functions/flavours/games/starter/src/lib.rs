pub trait Entity {
    fn name(&self) -> String;
    fn health(&self) -> u32;
}

pub struct Zombie {
    pub hp: u32,
}

pub struct Creeper {
    pub hp: u32,
}

pub struct Villager {
    pub name: String,
    pub hp: u32,
}

impl Entity for Zombie {
    fn name(&self) -> String {
        "zombie".to_string()
    }
    fn health(&self) -> u32 {
        self.hp
    }
}

impl Entity for Creeper {
    fn name(&self) -> String {
        "creeper".to_string()
    }
    fn health(&self) -> u32 {
        self.hp
    }
}

impl Entity for Villager {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn health(&self) -> u32 {
        self.hp
    }
}

/// The mob with the most health. `mobs` is never empty; ties go to the first.
pub fn strongest<T: Entity>(mobs: &[T]) -> &T {
    // TODO: keep a reference to the best mob so far, compare health()
    &mobs[0]
}

/// "zombie, zombie" for two zombies.
pub fn roster<T>(mobs: &[T]) -> String
where
    T: Entity,
{
    // TODO: turn each mob into its name(), then join with ", "
    todo!()
}

/// Health of all mobs added together.
pub fn total_health<T: Entity>(mobs: &[T]) -> u32 {
    // TODO: add up health() of every mob
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn villager(name: &str, hp: u32) -> Villager {
        Villager { name: name.to_string(), hp }
    }

    #[test]
    fn largest_works_for_many_types() {
        let zombies = [Zombie { hp: 5 }, Zombie { hp: 20 }, Zombie { hp: 12 }];
        assert_eq!(strongest(&zombies).health(), 20);
        let creepers = [Creeper { hp: 20 }, Creeper { hp: 1 }];
        assert_eq!(strongest(&creepers).health(), 20);
        let folk = [villager("Ada", 8), villager("Bo", 30)];
        assert_eq!(strongest(&folk).name(), "Bo");
    }

    #[test]
    fn show_all_joins_with_commas() {
        assert_eq!(roster(&[villager("Ada", 1), villager("Bo", 2)]), "Ada, Bo");
        assert_eq!(roster(&[Zombie { hp: 1 }, Zombie { hp: 2 }]), "zombie, zombie");
        assert_eq!(roster::<Creeper>(&[]), "");
    }

    #[test]
    fn count_matching_counts_equal_items() {
        let zombies = [Zombie { hp: 5 }, Zombie { hp: 20 }, Zombie { hp: 12 }];
        assert_eq!(total_health(&zombies), 37);
        assert_eq!(total_health(&[Creeper { hp: 20 }]), 20);
        assert_eq!(total_health::<Villager>(&[]), 0);
    }
}
