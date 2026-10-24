pub struct Roster {
    pub players: Vec<String>,
}

/// How many players are online.
pub fn count_players(roster: &Roster) -> usize {
    // TODO: give back how many players the roster holds
    todo!()
}

/// The length of every player name, added up (the width of the tab list).
pub fn total_chars(roster: &Roster) -> usize {
    // TODO: walk the names without taking them and add up their lengths
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roster() -> Roster {
        Roster {
            players: vec!["steve".to_string(), "alex".to_string(), "bob".to_string()],
        }
    }

    #[test]
    fn count_players_counts_online_names() {
        assert_eq!(count_players(&roster()), 3);
    }

    #[test]
    fn total_chars_sums_name_lengths() {
        let r = roster();
        assert_eq!(total_chars(&r), 12, "steve + alex + bob = 5 + 4 + 3");
        assert_eq!(r.players[0], "steve", "the server must still have its roster");
    }
}
