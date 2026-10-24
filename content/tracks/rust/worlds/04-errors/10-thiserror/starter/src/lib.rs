use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum TaskError {
    // TODO: the message "no priority"
    #[error("TODO")]
    Empty,
    // TODO: the message "priority is not a number: <the parse error>", and let `?` convert into it
    #[error("TODO")]
    BadPriority(ParseIntError),
    #[error("{0} is above 5")]
    TooHigh(u32),
}

/// Read a priority from 1 to 5.
pub fn parse_priority(text: &str) -> Result<u32, TaskError> {
    // TODO: trim; empty -> Empty; parse with `?` (the #[from] does the converting); above 5 -> TooHigh
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_come_from_the_attributes() {
        assert_eq!(TaskError::Empty.to_string(), "no priority");
        let parse_err = "high".parse::<u32>().unwrap_err();
        assert_eq!(
            TaskError::BadPriority(parse_err).to_string(),
            "priority is not a number: invalid digit found in string"
        );
        assert_eq!(TaskError::TooHigh(9).to_string(), "9 is above 5");
    }

    #[test]
    fn question_mark_converts_with_from() {
        assert_eq!(parse_priority(" 4 "), Ok(4));
        assert_eq!(parse_priority(""), Err(TaskError::Empty));
        assert_eq!(parse_priority("9"), Err(TaskError::TooHigh(9)));
        let parse_err = "high".parse::<u32>().unwrap_err();
        assert_eq!(parse_priority("high"), Err(TaskError::BadPriority(parse_err)));
    }
}
