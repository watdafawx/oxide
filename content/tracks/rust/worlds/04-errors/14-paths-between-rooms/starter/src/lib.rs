/// Minutes one task takes, set once for the whole crate. Private.
const MINUTES_PER_TASK: u32 = 30;

pub mod planner {
    /// How long `tasks` tasks take, in minutes.
    pub fn minutes_for(tasks: u32) -> u32 {
        // TODO: tasks times MINUTES_PER_TASK, which lives one room up
        todo!()
    }
}

// TODO: bring planner::minutes_for into this room with use

/// describe(4) -> "4 tasks take 120 minutes"
pub fn describe(tasks: u32) -> String {
    // TODO: build the sentence, calling minutes_for by its short name
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minutes_for_reads_the_constant_above() {
        assert_eq!(planner::minutes_for(4), 120);
        assert_eq!(planner::minutes_for(0), 0);
    }

    #[test]
    fn describe_calls_the_planner() {
        assert_eq!(describe(4), "4 tasks take 120 minutes");
    }
}
