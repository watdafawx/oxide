/// A plan: the minutes for one task, then the rest of the plan. `Done` ends it.
#[derive(Debug, PartialEq)]
pub enum Plan {
    Task(u32, Box<Plan>),
    Done,
}

/// The minutes of every task added up. An empty plan takes 0.
pub fn total_minutes(plan: &Plan) -> u32 {
    // TODO: match on plan: Done is 0, a Task is its minutes plus total_minutes of the rest
    todo!()
}

/// How many tasks the plan has.
pub fn task_count(plan: &Plan) -> u32 {
    // TODO: same shape as total_minutes, but each Task counts as 1
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn morning() -> Plan {
        Plan::Task(30, Box::new(Plan::Task(15, Box::new(Plan::Task(10, Box::new(Plan::Done))))))
    }

    #[test]
    fn total_minutes_adds_every_task() {
        assert_eq!(total_minutes(&morning()), 55, "30 + 15 + 10");
        assert_eq!(total_minutes(&Plan::Done), 0, "an empty plan takes no time");
    }

    #[test]
    fn task_count_counts_tasks() {
        assert_eq!(task_count(&morning()), 3);
        assert_eq!(task_count(&Plan::Done), 0, "an empty plan has no tasks");
    }
}
