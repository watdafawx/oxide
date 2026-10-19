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

fn player(name: &str, pos: (i32, i32, i32), items: &[&str]) -> Player {
    Player {
        name: name.to_string(),
        pos: BlockPos { x: pos.0, y: pos.1, z: pos.2 },
        inventory: items.iter().map(|s| s.to_string()).collect(),
    }
}

impl Server {
    /// Name of player number `i`, borrowed from the server.
    pub fn player_name(&self, i: usize) -> &str {
        &self.players[i].name
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
        let lines: Vec<String> = self
            .server
            .players
            .iter()
            .map(|p| {
                let mut line = format!("{} @ {},{},{}", p.name, p.pos.x, p.pos.y, p.pos.z);
                if !p.inventory.is_empty() {
                    line.push_str(&format!(" [items: {}]", p.inventory.join(", ")));
                }
                line
            })
            .collect();
        lines.join("; ")
    }

    /// Teleports the player called `name`. Returns their name.
    /// Nobody by that name: Err("No player named <name>.") and nobody moves.
    pub fn tp(&mut self, name: &str, to: BlockPos) -> Result<&str, String> {
        match self.server.players.iter_mut().find(|p| p.name == name) {
            Some(p) => {
                p.pos = to;
                Ok(p.name.as_str())
            }
            None => Err(format!("No player named {name}.")),
        }
    }

    /// Moves `item` from the stash into the inventory of the player called `name`.
    /// Unknown player: Err("No player named <name>."). Not in the stash: Err("There is no <item> in the stash.").
    pub fn give(&mut self, name: &str, item: &str) -> Result<(), String> {
        let player = self
            .server
            .players
            .iter_mut()
            .find(|p| p.name == name)
            .ok_or_else(|| format!("No player named {name}."))?;
        match self.server.stash.iter().position(|i| i == item) {
            Some(pos) => {
                let taken = self.server.stash.remove(pos);
                player.inventory.push(taken);
                Ok(())
            }
            None => Err(format!("There is no {item} in the stash.")),
        }
    }

    /// Removes the player called `name` from the server.
    /// Nobody by that name: Err("No player named <name>.").
    pub fn kick(&mut self, name: &str) -> Result<(), String> {
        match self.server.players.iter().position(|p| p.name == name) {
            Some(i) => {
                self.server.players.remove(i);
                Ok(())
            }
            None => Err(format!("No player named {name}.")),
        }
    }
}

/// Plays a script on a fresh demo console, one output line per command:
/// - "list"                  -> list()
/// - "tp <name> <x> <y> <z>" -> "Teleported <name> to x,y,z." or the error text
/// - "give <name> <item>"    -> "Gave <item> to <name>." or the error text
/// - "kick <name>"           -> "Kicked <name>." or the error text
/// - anything else (including coordinates that are not numbers) -> "Unknown command: <the whole line>"
pub fn run(script: &[&str]) -> Vec<String> {
    let mut console = Console::demo();
    let mut out = Vec::new();
    for &line in script {
        let words: Vec<&str> = line.split_whitespace().collect();
        let reply = match words.as_slice() {
            ["list"] => console.list(),
            ["tp", name, x, y, z] => {
                match (x.parse::<i32>(), y.parse::<i32>(), z.parse::<i32>()) {
                    (Ok(x), Ok(y), Ok(z)) => match console.tp(name, BlockPos { x, y, z }) {
                        Ok(who) => format!("Teleported {who} to {x},{y},{z}."),
                        Err(e) => e,
                    },
                    _ => format!("Unknown command: {line}"),
                }
            }
            ["give", name, item] => match console.give(name, item) {
                Ok(()) => format!("Gave {item} to {name}."),
                Err(e) => e,
            },
            ["kick", name] => match console.kick(name) {
                Ok(()) => format!("Kicked {name}."),
                Err(e) => e,
            },
            _ => format!("Unknown command: {line}"),
        };
        out.push(reply);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_name_borrows_from_world() {
        let console = Console::demo();
        assert_eq!(console.server.player_name(0), "Steve");
        assert_eq!(console.server.player_name(1), "Alex");
    }

    #[test]
    fn describe_shows_room_and_exits() {
        let mut console = Console::demo();
        console.server.players[1].inventory.clear();
        assert_eq!(console.list(), "Steve @ 0,64,0; Alex @ 10,70,-3");
    }

    #[test]
    fn describe_lists_items_when_present() {
        let mut console = Console::demo();
        assert_eq!(console.list(), "Steve @ 0,64,0; Alex @ 10,70,-3 [items: compass]");
        console.server.players[0].inventory.push("diamond".to_string());
        assert_eq!(
            console.list(),
            "Steve @ 0,64,0 [items: diamond]; Alex @ 10,70,-3 [items: compass]"
        );
    }

    #[test]
    fn go_moves_the_player() {
        let mut console = Console::demo();
        assert_eq!(console.tp("Steve", BlockPos { x: 5, y: 70, z: 5 }), Ok("Steve"));
        assert_eq!(console.server.players[0].pos, BlockPos { x: 5, y: 70, z: 5 });
        assert_eq!(console.tp("Alex", BlockPos { x: 0, y: 64, z: 0 }), Ok("Alex"));
        assert_eq!(console.server.players[1].pos, BlockPos { x: 0, y: 64, z: 0 });
    }

    #[test]
    fn go_blocked_keeps_player_in_place() {
        let mut console = Console::demo();
        let nowhere = BlockPos { x: 1, y: 1, z: 1 };
        assert_eq!(console.tp("Herobrine", nowhere), Err("No player named Herobrine.".to_string()));
        assert_eq!(console.server.players[0].pos, BlockPos { x: 0, y: 64, z: 0 }, "nobody may move");
        assert_eq!(console.server.players[1].pos, BlockPos { x: 10, y: 70, z: -3 }, "nobody may move");
    }

    #[test]
    fn take_moves_item_to_inventory() {
        let mut console = Console::demo();
        assert_eq!(console.give("Steve", "diamond"), Ok(()));
        assert_eq!(console.server.players[0].inventory, vec!["diamond".to_string()]);
        assert_eq!(console.server.stash, vec!["torch".to_string()], "the item must leave the stash");
    }

    #[test]
    fn take_missing_item_is_an_error() {
        let mut console = Console::demo();
        assert_eq!(console.give("Steve", "sword"), Err("There is no sword in the stash.".to_string()));
        assert!(console.server.players[0].inventory.is_empty());
        assert_eq!(console.server.stash, vec!["diamond".to_string(), "torch".to_string()]);
    }

    #[test]
    fn run_plays_a_whole_script() {
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
