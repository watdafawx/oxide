pub struct Roster {
    pub players: Vec<String>,
}

/// How many players are online.
pub fn count_players(roster: &Roster) -> usize {
    // TODO: return the length of roster.players
    todo!()
}

/// Sum of the length of every player name (the width of the tab list).
pub fn total_chars(roster: &Roster) -> usize {
    // TODO: add up name.len() for every name in roster.players
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
    fn count_items_counts() {
        assert_eq!(count_players(&roster()), 3);
    }

    #[test]
    fn total_chars_adds_lengths() {
        assert_eq!(total_chars(&roster()), 12, "steve + alex + bob = 5 + 4 + 3");
    }

    #[test]
    fn bag_is_still_usable_after_borrowing() {
        let r = roster();
        let n = count_players(&r);
        let c = total_chars(&r);
        assert_eq!((n, c), (3, 12));
        assert_eq!(r.players[0], "steve", "the roster must still be ours");
    }
}
