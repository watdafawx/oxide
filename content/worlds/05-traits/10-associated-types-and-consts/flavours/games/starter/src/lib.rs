use std::fmt::Display;

pub trait Mob {
    const ID: &str;
    type Loot;
    fn loot(&self) -> Self::Loot;
}

/// `looting` is the level of the Looting enchantment on the sword.
pub struct Zombie {
    pub looting: u32,
}

pub struct Creeper {
    pub charged: bool,
}

impl Mob for Zombie {
    // TODO: set ID to "zombie"
    const ID: &str = "";
    type Loot = u32;

    fn loot(&self) -> u32 {
        // TODO: rotten flesh dropped: 2 plus the looting level
        0
    }
}

impl Mob for Creeper {
    // TODO: set ID to "creeper"
    const ID: &str = "";
    type Loot = String;

    fn loot(&self) -> String {
        // TODO: "head" when charged, otherwise "gunpowder"
        String::new()
    }
}

/// "<ID>: <loot>", e.g. "zombie: 3".
pub fn label<M>(mob: &M) -> String
where
    M: Mob,
    M::Loot: Display,
{
    // TODO: format M::ID and mob.loot()
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zombie_loot_is_rotten_flesh_count() {
        assert_eq!(Zombie { looting: 0 }.loot(), 2);
        assert_eq!(Zombie { looting: 3 }.loot(), 5);
    }

    #[test]
    fn creeper_loot_is_gunpowder_or_head() {
        assert_eq!(Creeper { charged: false }.loot(), "gunpowder");
        assert_eq!(Creeper { charged: true }.loot(), "head");
    }

    #[test]
    fn mobs_set_their_ids() {
        assert_eq!(Zombie::ID, "zombie");
        assert_eq!(Creeper::ID, "creeper");
    }

    #[test]
    fn label_shows_id_and_loot() {
        assert_eq!(label(&Zombie { looting: 1 }), "zombie: 3");
        assert_eq!(label(&Creeper { charged: false }), "creeper: gunpowder");
    }
}
