/// One line of the quest list: `[x] Loops` when done, `[ ] Loops` when not.
pub fn quest_line(title: &str, done: bool) -> String {
    // TODO: pick the mark from `done` and put it in front of the title, with a space between
    todo!()
}

/// The header: `2 of 3 quests done` for a list of (title, done) pairs.
pub fn progress_summary(quests: &[(&str, bool)]) -> String {
    // TODO: count the pairs whose done is true, then build the sentence
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quest_line_shows_a_checkbox() {
        assert_eq!(quest_line("Loops", true), "[x] Loops");
        assert_eq!(quest_line("Loops", false), "[ ] Loops");
    }

    #[test]
    fn progress_summary_counts_finished_quests() {
        let quests = [("Variables", true), ("Loops", false), ("Functions", true)];
        assert_eq!(progress_summary(&quests), "2 of 3 quests done");
        assert_eq!(progress_summary(&[]), "0 of 0 quests done");
    }
}
