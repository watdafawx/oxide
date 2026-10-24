// TODO: a BlockPos is only numbers. Let it be cloned and copied.
#[derive(Debug, PartialEq)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// TODO: a Player owns a String name, so it can't be Copy. Let it be cloned.
#[derive(Debug, PartialEq)]
pub struct Player {
    pub name: String,
}

/// The same position twice: spawn and home.
pub fn spawn_and_home(pos: BlockPos) -> (BlockPos, BlockPos) {
    (pos, pos)
}

/// A player record and a backup, each owning its own name.
pub fn backup(player: Player) -> (Player, Player) {
    (player.clone(), player)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_and_home_returns_pos_twice() {
        let pos = BlockPos { x: 3, y: 64, z: -4 };
        assert_eq!(
            spawn_and_home(pos),
            (BlockPos { x: 3, y: 64, z: -4 }, BlockPos { x: 3, y: 64, z: -4 })
        );
    }

    #[test]
    fn backup_gives_two_equal_players() {
        let (a, b) = backup(Player { name: "Steve".to_string() });
        assert_eq!(a, b);
        assert_eq!(a.name, "Steve");
        assert_ne!(a.name.as_ptr(), b.name.as_ptr(), "each record owns its own name");
    }
}
