/// Adds " [OP]" to the end of the caller's player name.
pub fn tag_as_op(name: &mut String) {
    name.push_str(" [OP]");
}

/// Queues the same console command two times.
pub fn queue_twice(queue: &mut Vec<String>, command: &str) {
    queue.push(command.to_string());
    queue.push(command.to_string());
}

/// Kicks the last player in `online`: they move to the end of `kicked`. Does nothing if nobody is online.
pub fn kick_last(online: &mut Vec<String>, kicked: &mut Vec<String>) {
    if let Some(player) = online.pop() {
        kicked.push(player);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_as_op_appends_tag() {
        let mut name = String::from("steve");
        tag_as_op(&mut name);
        assert_eq!(name, "steve [OP]");
    }

    #[test]
    fn queue_twice_pushes_command_twice() {
        let mut queue = vec!["list".to_string()];
        queue_twice(&mut queue, "save-all");
        assert_eq!(queue, vec!["list", "save-all", "save-all"]);
    }

    #[test]
    fn kick_last_moves_player_to_kicked_list() {
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
