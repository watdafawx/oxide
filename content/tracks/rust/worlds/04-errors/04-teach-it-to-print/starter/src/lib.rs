use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PriorityError {
    Empty,
    NotANumber(String),
    TooHigh(u32),
}

impl fmt::Display for PriorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: write one message per variant into f:
        //       Empty -> "no priority", NotANumber(text) -> "'<text>' is not a number", TooHigh(n) -> "<n> is above 5"
        todo!()
    }
}

/// From the last quests. Given: you don't need to change it.
pub fn parse_priority(text: &str) -> Result<u32, PriorityError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(PriorityError::Empty);
    }
    let n = match text.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(PriorityError::NotANumber(text.to_string())),
    };
    if n > 5 {
        return Err(PriorityError::TooHigh(n));
    }
    Ok(n)
}

/// explain("3") -> "priority 3", explain("high") -> "skipped: 'high' is not a number"
pub fn explain(text: &str) -> String {
    // TODO: a good priority gives "priority <n>"; an error gives "skipped: " followed by the error's message
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_error_prints_a_message() {
        assert_eq!(PriorityError::Empty.to_string(), "no priority");
        assert_eq!(PriorityError::NotANumber("high".to_string()).to_string(), "'high' is not a number");
        assert_eq!(PriorityError::TooHigh(9).to_string(), "9 is above 5");
    }

    #[test]
    fn explain_puts_the_error_in_a_sentence() {
        assert_eq!(explain("3"), "priority 3");
        assert_eq!(explain("high"), "skipped: 'high' is not a number");
        assert_eq!(explain(""), "skipped: no priority");
    }
}
