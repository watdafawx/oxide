#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct Player {
    pub name: String,
    pub pos: BlockPos,
    pub inventory: Vec<String>,
}

/// Owns every player and the admin stash that `give` hands items out of.
pub struct Server {
    pub players: Vec<Player>,
    pub stash: Vec<String>,
}

/// The console: it owns the server, and every command line changes its state.
pub struct Console {
    pub server: Server,
}

/// Builds one player, turning the borrowed text into owned Strings.
fn player(name: &str, pos: (i32, i32, i32), items: &[&str]) -> Player {
    let mut p = Player {
        name: name.to_string(),
        pos: BlockPos { x: pos.0, y: pos.1, z: pos.2 },
        inventory: Vec::new(),
    };
    for item in items {
        p.inventory.push(item.to_string());
    }
    p
}

/// "5 70 -3" -> Some(BlockPos { x: 5, y: 70, z: -3 }). Anything else (not three whole numbers) -> None.
pub fn parse_pos(text: &str) -> Option<BlockPos> {
    let mut nums = Vec::new();
    for word in text.split_whitespace() {
        match word.parse::<i32>() {
            Ok(n) => nums.push(n),
            Err(_) => return None,
        }
    }
    if nums.len() == 3 {
        Some(BlockPos { x: nums[0], y: nums[1], z: nums[2] })
    } else {
        None
    }
}

impl Server {
    /// Name of player number `i`, borrowed from the server.
    pub fn player_name(&self, i: usize) -> &str {
        // TODO: lend out the name of player number i
        todo!()
    }
}

impl Console {
    /// Steve at 0,64,0 with an empty inventory, Alex at 10,70,-3 holding a compass,
    /// and a stash with a diamond and a torch.
    pub fn demo() -> Console {
        Console {
            server: Server {
                players: vec![
                    player("Steve", (0, 64, 0), &[]),
                    player("Alex", (10, 70, -3), &["compass"]),
                ],
                stash: vec!["diamond".to_string(), "torch".to_string()],
            },
        }
    }

    /// "Steve @ 0,64,0; Alex @ 10,70,-3 [items: compass]"
    /// Each player's `[items: ...]` part is left out when their inventory is empty.
    pub fn list(&self) -> String {
        // TODO: borrow each player and build "<name> @ x,y,z" (plus " [items: a, b]" when they
        // hold something); collect the lines in a list and join them with "; "
        todo!()
    }

    /// Teleports the player called `name`. Gives back their name.
    /// Nobody by that name: Err("No player named <name>.") and nobody moves.
    pub fn tp(&mut self, name: &str, to: BlockPos) -> Result<&str, String> {
        // TODO: walk the players mutably; for the one called `name`, set their position
        // and lend out their name
        todo!()
    }

    /// Moves `item` from the stash into the inventory of the player called `name`.
    /// Unknown player: Err("No player named <name>."). Not in the stash: Err("There is no <item> in the stash.").
    pub fn give(&mut self, name: &str, item: &str) -> Result<(), String> {
        // TODO: find the player's position in the list, then the item's position in the stash;
        // remove the item and push the owned String onto that player's inventory
        todo!()
    }

    /// Removes the player called `name` from the server.
    /// Nobody by that name: Err("No player named <name>.").
    pub fn kick(&mut self, name: &str) -> Result<(), String> {
        // TODO: find the player's position and remove them
        todo!()
    }
}

/// Plays a script on a fresh demo console, one output line per command:
/// - "list"                  -> list()
/// - "tp <name> <x> <y> <z>" -> "Teleported <name> to x,y,z." or the error text
/// - "give <name> <item>"    -> "Gave <item> to <name>." or the error text
/// - "kick <name>"           -> "Kicked <name>." or the error text
/// - anything else (including coordinates that are not numbers) -> "Unknown command: <the whole line>"
pub fn run(script: &[&str]) -> Vec<String> {
    // TODO: make a console, then for each line split off the command word (split_once),
    // match on it, split the rest again where needed, and push one reply per line
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_name_borrows_from_server() {
        let console = Console::demo();
        assert_eq!(console.server.player_name(0), "Steve");
        assert_eq!(console.server.player_name(1), "Alex");
    }

    #[test]
    fn list_shows_every_player_position() {
        let mut console = Console::demo();
        console.server.players[1].inventory.clear();
        assert_eq!(console.list(), "Steve @ 0,64,0; Alex @ 10,70,-3");
    }

    #[test]
    fn list_adds_items_only_for_holders() {
        let mut console = Console::demo();
        assert_eq!(console.list(), "Steve @ 0,64,0; Alex @ 10,70,-3 [items: compass]");
        console.server.players[0].inventory.push("diamond".to_string());
        assert_eq!(
            console.list(),
            "Steve @ 0,64,0 [items: diamond]; Alex @ 10,70,-3 [items: compass]"
        );
    }

    #[test]
    fn tp_moves_the_player() {
        let mut console = Console::demo();
        assert_eq!(console.tp("Steve", BlockPos { x: 5, y: 70, z: 5 }), Ok("Steve"));
        assert_eq!(console.server.players[0].pos, BlockPos { x: 5, y: 70, z: 5 });
        assert_eq!(console.tp("Alex", BlockPos { x: 0, y: 64, z: 0 }), Ok("Alex"));
        assert_eq!(console.server.players[1].pos, BlockPos { x: 0, y: 64, z: 0 });
    }

    #[test]
    fn tp_unknown_player_moves_nobody() {
        let mut console = Console::demo();
        let nowhere = BlockPos { x: 1, y: 1, z: 1 };
        assert_eq!(console.tp("Herobrine", nowhere), Err("No player named Herobrine.".to_string()));
        assert_eq!(console.server.players[0].pos, BlockPos { x: 0, y: 64, z: 0 }, "nobody may move");
        assert_eq!(console.server.players[1].pos, BlockPos { x: 10, y: 70, z: -3 }, "nobody may move");
    }

    #[test]
    fn give_moves_stash_item_to_inventory() {
        let mut console = Console::demo();
        assert_eq!(console.give("Steve", "diamond"), Ok(()));
        assert_eq!(console.server.players[0].inventory, vec!["diamond".to_string()]);
        assert_eq!(console.server.stash, vec!["torch".to_string()], "the item must leave the stash");
    }

    #[test]
    fn give_missing_item_is_an_error() {
        let mut console = Console::demo();
        assert_eq!(console.give("Steve", "sword"), Err("There is no sword in the stash.".to_string()));
        assert!(console.server.players[0].inventory.is_empty());
        assert_eq!(console.server.stash, vec!["diamond".to_string(), "torch".to_string()]);
    }

    #[test]
    fn run_plays_a_whole_console_script() {
        let out = run(&[
            "give Steve diamond",
            "tp Steve 5 70 5",
            "tp Herobrine 0 0 0",
            "give Steve sword",
            "kick Alex",
            "kick Alex",
            "list",
            "dance",
        ]);
        assert_eq!(
            out,
            vec![
                "Gave diamond to Steve.",
                "Teleported Steve to 5,70,5.",
                "No player named Herobrine.",
                "There is no sword in the stash.",
                "Kicked Alex.",
                "No player named Alex.",
                "Steve @ 5,70,5 [items: diamond]",
                "Unknown command: dance",
            ]
        );
    }
}
