use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TaskError {
    TooHigh(u32),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskError::TooHigh(n) => write!(f, "{n} is above 5"),
        }
    }
}

impl std::error::Error for TaskError {}

/// Priorities go from 1 to 5.
pub fn check_priority(n: u32) -> Result<u32, TaskError> {
    if n > 5 {
        return Err(TaskError::TooHigh(n));
    }
    Ok(n)
}

/// Read and check a priority. Two kinds of error, one box. Given: you don't need to change it.
pub fn load_priority(text: &str) -> Result<u32, Box<dyn Error>> {
    let n: u32 = text.trim().parse()?;
    let n = check_priority(n)?;
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_good_priority_loads() {
        assert_eq!(load_priority(" 3 ").unwrap(), 3);
    }

    #[test]
    fn your_error_travels_in_the_box() {
        let err = load_priority("9").unwrap_err();
        assert_eq!(err.to_string(), "9 is above 5");
        assert_eq!(err.downcast_ref::<TaskError>(), Some(&TaskError::TooHigh(9)), "the box still holds your TaskError");
    }
}
