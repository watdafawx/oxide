pub struct Room {
    pub name: String,
    pub description: String,
    /// (direction, index of the room it leads to)
    pub exits: Vec<(String, usize)>,
    pub items: Vec<String>,
}

pub struct World {
    pub rooms: Vec<Room>,
}

pub struct Player {
    /// Index into World::rooms
    pub room: usize,
    pub inventory: Vec<String>,
}

pub struct Game {
    pub world: World,
    pub player: Player,
}

/// Builds one room, turning the borrowed text into owned Strings.
fn room(name: &str, description: &str, exits: &[(&str, usize)], items: &[&str]) -> Room {
    let mut r = Room {
        name: name.to_string(),
        description: description.to_string(),
        exits: Vec::new(),
        items: Vec::new(),
    };
    for exit in exits {
        r.exits.push((exit.0.to_string(), exit.1));
    }
    for item in items {
        r.items.push(item.to_string());
    }
    r
}

impl World {
    /// Name of room number `i`, borrowed from the world.
    pub fn room_name(&self, i: usize) -> &str {
        &self.rooms[i].name
    }
}

impl Game {
    /// Cellar(0) -north-> Hall(1) -east-> Library(2). The player starts in the Cellar.
    pub fn demo() -> Game {
        Game {
            world: World {
                rooms: vec![
                    room("Cellar", "A damp cellar.", &[("north", 1)], &["lamp"]),
                    room("Hall", "A long hall.", &[("south", 0), ("east", 2)], &[]),
                    room("Library", "Dusty shelves.", &[("west", 1)], &["key", "book"]),
                ],
            },
            player: Player {
                room: 0,
                inventory: Vec::new(),
            },
        }
    }

    /// "Cellar: A damp cellar. [exits: north] [items: lamp]"
    /// The `[items: ...]` part is left out when the room has no items.
    pub fn describe(&self) -> String {
        let here = &self.world.rooms[self.player.room];
        let mut exits: Vec<&str> = Vec::new();
        for exit in &here.exits {
            exits.push(&exit.0);
        }
        let mut text = format!("{}: {} [exits: {}]", here.name, here.description, exits.join(", "));
        if !here.items.is_empty() {
            text.push_str(&format!(" [items: {}]", here.items.join(", ")));
        }
        text
    }

    /// Walks through the exit named `dir`. Gives back the name of the new room.
    /// Unknown direction: Err("You can't go <dir>.") and the player stays put.
    pub fn go(&mut self, dir: &str) -> Result<&str, String> {
        let here = &self.world.rooms[self.player.room];
        let mut target = None;
        for exit in &here.exits {
            if exit.0 == dir {
                target = Some(exit.1);
            }
        }
        match target {
            Some(i) => {
                self.player.room = i;
                Ok(self.world.room_name(i))
            }
            None => Err(format!("You can't go {dir}.")),
        }
    }

    /// Moves `item` from the current room into the inventory.
    /// Not here: Err("There is no <item> here.").
    pub fn take(&mut self, item: &str) -> Result<(), String> {
        let here = &mut self.world.rooms[self.player.room];
        for i in 0..here.items.len() {
            if here.items[i] == item {
                let taken = here.items.remove(i);
                self.player.inventory.push(taken);
                return Ok(());
            }
        }
        Err(format!("There is no {item} here."))
    }
}

/// Plays a script on a fresh demo game, one output line per command:
/// - "look"       -> describe()
/// - "go <dir>"   -> "You walk to <room>." or the error text
/// - "take <it>"  -> "Taken: <it>" or the error text
/// - anything else -> "Unknown command: <the whole line>"
pub fn run(script: &[&str]) -> Vec<String> {
    let mut game = Game::demo();
    let mut out = Vec::new();
    for line in script {
        let (cmd, arg) = match line.split_once(' ') {
            Some(parts) => parts,
            None => (*line, ""),
        };
        let reply = match cmd {
            "look" => game.describe(),
            "go" => match game.go(arg) {
                Ok(name) => format!("You walk to {name}."),
                Err(e) => e,
            },
            "take" => match game.take(arg) {
                Ok(()) => format!("Taken: {arg}"),
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
        let game = Game::demo();
        assert_eq!(game.world.room_name(0), "Cellar");
        assert_eq!(game.world.room_name(2), "Library");
    }

    #[test]
    fn describe_shows_room_and_exits() {
        let mut game = Game::demo();
        game.world.rooms[0].items.clear();
        assert_eq!(game.describe(), "Cellar: A damp cellar. [exits: north]");
        game.player.room = 1;
        assert_eq!(game.describe(), "Hall: A long hall. [exits: south, east]");
    }

    #[test]
    fn describe_lists_items_when_present() {
        let mut game = Game::demo();
        assert_eq!(game.describe(), "Cellar: A damp cellar. [exits: north] [items: lamp]");
        game.player.room = 2;
        assert_eq!(game.describe(), "Library: Dusty shelves. [exits: west] [items: key, book]");
    }

    #[test]
    fn go_moves_the_player() {
        let mut game = Game::demo();
        assert_eq!(game.go("north"), Ok("Hall"));
        assert_eq!(game.player.room, 1);
        assert_eq!(game.go("east"), Ok("Library"));
        assert_eq!(game.player.room, 2);
    }

    #[test]
    fn go_blocked_keeps_player_in_place() {
        let mut game = Game::demo();
        assert_eq!(game.go("west"), Err("You can't go west.".to_string()));
        assert_eq!(game.player.room, 0, "a blocked move must not change the room");
    }

    #[test]
    fn take_moves_item_to_inventory() {
        let mut game = Game::demo();
        assert_eq!(game.take("lamp"), Ok(()));
        assert_eq!(game.player.inventory, vec!["lamp".to_string()]);
        assert!(game.world.rooms[0].items.is_empty(), "the item must leave the room");
    }

    #[test]
    fn take_missing_item_is_an_error() {
        let mut game = Game::demo();
        assert_eq!(game.take("sword"), Err("There is no sword here.".to_string()));
        assert!(game.player.inventory.is_empty());
        assert_eq!(game.world.rooms[0].items, vec!["lamp".to_string()]);
    }

    #[test]
    fn run_plays_a_whole_script() {
        let out = run(&[
            "take lamp",
            "go north",
            "go east",
            "take key",
            "take lamp",
            "dance",
            "look",
        ]);
        assert_eq!(
            out,
            vec![
                "Taken: lamp",
                "You walk to Hall.",
                "You walk to Library.",
                "Taken: key",
                "There is no lamp here.",
                "Unknown command: dance",
                "Library: Dusty shelves. [exits: west] [items: book]",
            ]
        );
    }
}
