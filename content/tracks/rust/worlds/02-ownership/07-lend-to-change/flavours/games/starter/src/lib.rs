/// Adds " [OP]" to the end of the caller's player name.
pub fn tag_as_op(name: &mut String) {
    // TODO: add " [OP]" to the end of name
    todo!()
}

/// Queues the same console command two times.
pub fn queue_twice(queue: &mut Vec<String>, command: &str) {
    // TODO: push command two times, each as an owned String
    todo!()
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
}
