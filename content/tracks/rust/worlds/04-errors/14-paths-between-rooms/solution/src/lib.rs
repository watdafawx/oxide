/// Minutes one task takes, set once for the whole crate. Private.
const MINUTES_PER_TASK: u32 = 30;

pub mod planner {
    /// How long `tasks` tasks take, in minutes.
    pub fn minutes_for(tasks: u32) -> u32 {
        tasks * super::MINUTES_PER_TASK
    }
}

use planner::minutes_for;

/// describe(4) -> "4 tasks take 120 minutes"
pub fn describe(tasks: u32) -> String {
    format!("{tasks} tasks take {} minutes", minutes_for(tasks))
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
