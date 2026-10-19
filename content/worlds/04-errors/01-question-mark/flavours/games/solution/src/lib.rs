use std::num::ParseIntError;

/// A manifest says `mods = 21`. Client and server both load every mod, so double it.
pub fn mods_for_client_and_server(count: &str) -> Result<i32, ParseIntError> {
    let n = count.parse::<i32>()?;
    Ok(n * 2)
}

/// First letter of a mod name, uppercased, for the sorted mod list. None for an empty name.
pub fn mod_initial(name: &str) -> Option<char> {
    let c = name.chars().next()?;
    c.to_uppercase().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_double_doubles_a_number() {
        assert_eq!(mods_for_client_and_server("21"), Ok(42));
        assert_eq!(mods_for_client_and_server("-4"), Ok(-8));
    }

    #[test]
    fn parse_and_double_passes_the_error_up() {
        assert!(mods_for_client_and_server("21x").is_err(), "\"21x\" is not a mod count");
        assert!(mods_for_client_and_server("").is_err(), "empty string is not a mod count");
    }

    #[test]
    fn first_char_upper_uses_question_mark_on_option() {
        assert_eq!(mod_initial("sodium"), Some('S'));
        assert_eq!(mod_initial(""), None, "empty name has no first letter");
    }
}
