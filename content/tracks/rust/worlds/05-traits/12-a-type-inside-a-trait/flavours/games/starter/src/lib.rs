use std::fmt::Display;

pub trait Mob {
    type Loot;
    fn loot(&self) -> Self::Loot;
}

pub struct Zombie {
    pub looting: u32,
}

pub struct Creeper {
    pub charged: bool,
}

impl Mob for Zombie {
    type Loot = u32;

    fn loot(&self) -> u32 {
        // TODO: 2 rotten flesh, plus one per looting level
        0
    }
}

impl Mob for Creeper {
    type Loot = String;

    fn loot(&self) -> String {
        // TODO: "head" when charged, otherwise "gunpowder"
        String::new()
    }
}

/// "drops: <loot>", e.g. "drops: 3".
pub fn drop_line<M>(mob: &M) -> String
where
    M: Mob,
    M::Loot: Display,
{
    // TODO: return "drops: <loot>" using mob.loot()
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zombie_loot_is_a_count() {
        assert_eq!(Zombie { looting: 0 }.loot(), 2);
        assert_eq!(Zombie { looting: 3 }.loot(), 5);
    }

    #[test]
    fn creeper_loot_is_an_item_name() {
        assert_eq!(Creeper { charged: false }.loot(), "gunpowder");
        assert_eq!(Creeper { charged: true }.loot(), "head");
    }

    #[test]
    fn drop_line_prints_any_loot() {
        assert_eq!(drop_line(&Zombie { looting: 1 }), "drops: 3");
        assert_eq!(drop_line(&Creeper { charged: false }), "drops: gunpowder");
    }
}
