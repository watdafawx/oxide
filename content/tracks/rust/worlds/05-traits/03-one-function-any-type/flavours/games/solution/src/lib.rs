pub trait Entity {
    fn health(&self) -> u32;
}

pub struct Zombie {
    pub hp: u32,
}

pub struct Creeper {
    pub hp: u32,
}

impl Entity for Zombie {
    fn health(&self) -> u32 {
        self.hp
    }
}

impl Entity for Creeper {
    fn health(&self) -> u32 {
        self.hp
    }
}

/// Health of the whole wave added together.
pub fn total_health<T: Entity>(mobs: &[T]) -> u32 {
    mobs.iter().map(|m| m.health()).sum()
}

/// The mob with the most health. `mobs` is never empty; ties go to the first.
pub fn strongest<T: Entity>(mobs: &[T]) -> &T {
    let mut best = &mobs[0];
    for m in mobs {
        if m.health() > best.health() {
            best = m;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_health_sums_any_mob() {
        let zombies = [Zombie { hp: 5 }, Zombie { hp: 20 }, Zombie { hp: 12 }];
        assert_eq!(total_health(&zombies), 37);
        assert_eq!(total_health(&[Creeper { hp: 20 }]), 20);
        assert_eq!(total_health::<Creeper>(&[]), 0);
    }

    #[test]
    fn strongest_picks_the_most_health() {
        let zombies = [Zombie { hp: 5 }, Zombie { hp: 20 }, Zombie { hp: 12 }];
        assert_eq!(strongest(&zombies).hp, 20);
        let creepers = [Creeper { hp: 20 }, Creeper { hp: 1 }];
        assert_eq!(strongest(&creepers).hp, 20);
    }
}
