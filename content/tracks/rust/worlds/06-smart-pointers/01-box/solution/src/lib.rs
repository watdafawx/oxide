/// A plan: the minutes for one task, then the rest of the plan. `Done` ends it.
#[derive(Debug, PartialEq)]
pub enum Plan {
    Task(u32, Box<Plan>),
    Done,
}

/// A longer plan: a new task of `minutes` in front of `plan`.
pub fn add_first(plan: Plan, minutes: u32) -> Plan {
    Plan::Task(minutes, Box::new(plan))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_two_task_plan_builds() {
        let plan = Plan::Task(30, Box::new(Plan::Task(15, Box::new(Plan::Done))));
        match plan {
            Plan::Task(first, _) => assert_eq!(first, 30, "the first task takes 30 minutes"),
            Plan::Done => panic!("the plan should start with a task"),
        }
    }

    #[test]
    fn add_first_puts_new_task_in_front() {
        let plan = add_first(Plan::Task(15, Box::new(Plan::Done)), 30);
        assert_eq!(plan, Plan::Task(30, Box::new(Plan::Task(15, Box::new(Plan::Done)))));
    }
}
