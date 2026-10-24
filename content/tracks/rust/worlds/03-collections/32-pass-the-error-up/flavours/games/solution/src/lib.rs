use std::num::ParseIntError;

/// How many items `stacks` stacks of `size` hold: ("3", "64") -> Ok(192).
/// An argument that is not a number comes back as the parse error.
pub fn items_in(stacks: &str, size: &str) -> Result<u32, ParseIntError> {
    let stacks = stacks.parse::<u32>()?;
    let size = size.parse::<u32>()?;
    Ok(stacks * size)
}

/// The first two hotbar stacks added up. None if there are fewer than two.
pub fn first_two_slots(hotbar: &[u32]) -> Option<u32> {
    let a = hotbar.get(0)?;
    let b = hotbar.get(1)?;
    Some(a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_in_multiplies_two_parsed_numbers() {
        assert_eq!(items_in("3", "64"), Ok(192));
        assert_eq!(items_in("0", "16"), Ok(0));
    }

    #[test]
    fn items_in_passes_a_bad_number_up() {
        assert!(items_in("three", "64").is_err(), "\"three\" is not a number");
        assert!(items_in("3", "-1").is_err(), "a stack size has no minus");
    }

    #[test]
    fn first_two_slots_uses_question_mark_on_option() {
        assert_eq!(first_two_slots(&[1, 16, 8]), Some(17));
        assert_eq!(first_two_slots(&[1]), None, "one slot is not two");
    }
}
