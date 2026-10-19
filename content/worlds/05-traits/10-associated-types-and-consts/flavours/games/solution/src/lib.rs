use std::fmt::Display;

pub trait Mob {
    const ID: &str;
    type Drop;
    fn loot(&self) -> Self::Drop;
}

/// `looting` is the level of the Looting enchantment on the sword.
pub struct Zombie {
    pub looting: u32,
}

pub struct Creeper {
    pub charged: bool,
}

impl Mob for Zombie {
    const ID: &str = "zombie";
    type Drop = u32;

    fn loot(&self) -> u32 {
        2 + self.looting
    }
}

impl Mob for Creeper {
    const ID: &str = "creeper";
    type Drop = String;

    fn loot(&self) -> String {
        if self.charged { "head" } else { "gunpowder" }.to_string()
    }
}

/// "<ID>: <loot>", e.g. "zombie: 3".
pub fn label<M>(mob: &M) -> String
where
    M: Mob,
    M::Drop: Display,
{
    format!("{}: {}", M::ID, mob.loot())
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
