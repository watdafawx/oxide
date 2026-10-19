// TODO: this type needs `Clone` (it holds a String, so not Copy).
#[derive(Debug, PartialEq)]
pub struct Player {
    pub name: String,
}

// TODO: this type is only numbers, so it can be `Clone` and `Copy`.
#[derive(Debug, PartialEq)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Two independent copies of the player record.
pub fn duplicate_player(player: &Player) -> Vec<Player> {
    // TODO: return a Vec with two clones of player
    todo!()
}

/// The same position twice (spawn and home).
pub fn spawn_and_home(pos: BlockPos) -> (BlockPos, BlockPos) {
    // TODO: return pos twice (this needs Copy on BlockPos)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_player_returns_two_copies() {
        let player = Player { name: "Steve".to_string() };
        let copies = duplicate_player(&player);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0], player);
        assert_eq!(copies[1], player);
    }

    #[test]
    fn spawn_and_home_returns_pos_twice() {
        let pos = BlockPos { x: 3, y: 64, z: -4 };
        assert_eq!(
            spawn_and_home(pos),
            (BlockPos { x: 3, y: 64, z: -4 }, BlockPos { x: 3, y: 64, z: -4 })
        );
    }
}
