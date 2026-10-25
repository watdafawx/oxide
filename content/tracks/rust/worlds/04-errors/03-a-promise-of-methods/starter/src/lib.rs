/// A promise: any type that keeps it can describe itself in one short line.
pub trait Label {
    fn label(&self) -> String;
}

pub enum Status {
    Open,
    Done,
}

impl Label for Status {
    fn label(&self) -> String {
        match self {
            Status::Open => "open".to_string(),
            Status::Done => "done".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PriorityError {
    Empty,
    NotANumber(String),
    TooHigh(u32),
}

// TODO: keep the Label promise for PriorityError:
//       Empty -> "no priority", NotANumber(text) -> "'<text>' is not a number", TooHigh(n) -> "<n> is above 5"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_priority_is_labelled() {
        assert_eq!(Status::Done.label(), "done", "the given Status impl, for comparison");
        assert_eq!(PriorityError::Empty.label(), "no priority");
    }

    #[test]
    fn labels_show_what_was_typed() {
        assert_eq!(PriorityError::NotANumber("high".to_string()).label(), "'high' is not a number");
        assert_eq!(PriorityError::TooHigh(9).label(), "9 is above 5");
    }
}
