use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum TaskError {
    BadPriority(ParseIntError),
    TooHigh(u32),
}

impl From<ParseIntError> for TaskError {
    fn from(e: ParseIntError) -> Self {
        // TODO: wrap the parse error `e` in the TaskError variant for bad priorities
        todo!()
    }
}

/// Read a priority from 1 to 5. The `?` uses the From impl above. Given: you don't need to change it.
pub fn parse_priority(text: &str) -> Result<u32, TaskError> {
    let n: u32 = text.trim().parse()?;
    if n > 5 {
        return Err(TaskError::TooHigh(n));
    }
    Ok(n)
}

/// Add up the priority of every line. The first broken line fails the whole total.
pub fn total_priority(lines: &[&str]) -> Result<u32, TaskError> {
    // TODO: add up parse_priority of every line, letting `?` hand back the first error
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_wraps_the_parse_error() {
        let parse_err = "high".parse::<u32>().unwrap_err();
        assert_eq!(TaskError::from(parse_err.clone()), TaskError::BadPriority(parse_err));
    }

    #[test]
    fn total_priority_stops_at_the_first_error() {
        assert_eq!(total_priority(&["1", "2", " 3 "]), Ok(6));
        assert_eq!(total_priority(&["1", "9", "high"]), Err(TaskError::TooHigh(9)));
        let parse_err = "high".parse::<u32>().unwrap_err();
        assert_eq!(total_priority(&["1", "high"]), Err(TaskError::BadPriority(parse_err)));
    }
}
