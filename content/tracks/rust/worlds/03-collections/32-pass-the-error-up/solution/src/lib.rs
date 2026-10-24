use std::num::ParseIntError;

/// The value of one stock line: qty times price in cents. ("3", "150") -> Ok(450).
/// A field that is not a number comes back as the parse error.
pub fn line_value(qty: &str, price: &str) -> Result<u32, ParseIntError> {
    let qty = qty.parse::<u32>()?;
    let price = price.parse::<u32>()?;
    Ok(qty * price)
}

/// The first two counts added up. None if there are fewer than two.
pub fn first_two_total(counts: &[u32]) -> Option<u32> {
    let a = counts.get(0)?;
    let b = counts.get(1)?;
    Some(a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_value_multiplies_two_parsed_numbers() {
        assert_eq!(line_value("3", "150"), Ok(450));
        assert_eq!(line_value("0", "999"), Ok(0));
    }

    #[test]
    fn line_value_passes_a_bad_number_up() {
        assert!(line_value("three", "150").is_err(), "\"three\" is not a number");
        assert!(line_value("3", "1.50").is_err(), "u32 has no decimals");
    }

    #[test]
    fn first_two_total_uses_question_mark_on_option() {
        assert_eq!(first_two_total(&[12, 6, 1]), Some(18));
        assert_eq!(first_two_total(&[12]), None, "one count is not two");
    }
}
