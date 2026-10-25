use std::fmt::Display;

pub trait Mob {
    const ID: &str;
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
    const ID: &str = "minecraft:zombie";
    type Loot = u32;

    fn loot(&self) -> u32 {
        2 + self.looting
    }
}

impl Mob for Creeper {
    const ID: &str = "minecraft:creeper";
    type Loot = String;

    fn loot(&self) -> String {
        if self.charged { "head" } else { "gunpowder" }.to_string()
    }
}

/// "<ID>: <loot>", e.g. "minecraft:zombie: 3".
pub fn spawn_line<M>(mob: &M) -> String
where
    M: Mob,
    M::Loot: Display,
{
    format!("{}: {}", M::ID, mob.loot())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobs_set_their_ids() {
        assert_eq!(Zombie::ID, "minecraft:zombie");
        assert_eq!(Creeper::ID, "minecraft:creeper");
    }

    #[test]
    fn spawn_line_uses_id_and_loot() {
        assert_eq!(spawn_line(&Zombie { looting: 1 }), "minecraft:zombie: 3");
        assert_eq!(spawn_line(&Creeper { charged: true }), "minecraft:creeper: head");
    }
}
