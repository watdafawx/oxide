/// Adds " [OP]" to the end of the caller's player name.
pub fn tag_as_op(name: &mut String) {
    // TODO: push " [OP]" onto name
    todo!()
}

/// Queues the same console command two times.
pub fn queue_twice(queue: &mut Vec<String>, command: &str) {
    // TODO: push command (as an owned String) two times
    todo!()
}

/// Kicks the last player in `online`: they move to the end of `kicked`. Does nothing if nobody is online.
pub fn kick_last(online: &mut Vec<String>, kicked: &mut Vec<String>) {
    // TODO: pop from `online`; if there was a player, push them onto `kicked`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_exclaim_appends() {
        let mut name = String::from("steve");
        tag_as_op(&mut name);
        assert_eq!(name, "steve [OP]");
    }

    #[test]
    fn push_twice_adds_two() {
        let mut queue = vec!["list".to_string()];
        queue_twice(&mut queue, "save-all");
        assert_eq!(queue, vec!["list", "save-all", "save-all"]);
    }

    #[test]
    fn move_last_between_vecs() {
        let mut online = vec!["steve".to_string(), "alex".to_string(), "bob".to_string()];
        let mut kicked = vec!["griefer".to_string()];
        kick_last(&mut online, &mut kicked);
        assert_eq!(online, vec!["steve", "alex"]);
        assert_eq!(kicked, vec!["griefer", "bob"]);
        let mut nobody = Vec::new();
        let mut list = vec!["alex".to_string()];
        kick_last(&mut nobody, &mut list);
        assert_eq!(list, vec!["alex"], "nobody to kick from an empty server");
    }
}
