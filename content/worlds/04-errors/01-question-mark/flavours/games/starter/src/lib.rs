use std::num::ParseIntError;

/// A manifest says `mods = 21`. Client and server both load every mod, so double it.
pub fn mods_for_client_and_server(count: &str) -> Result<i32, ParseIntError> {
    // TODO: parse `count` as i32 with `?`, then return Ok(n * 2)
    todo!()
}

/// First letter of a mod name, uppercased, for the sorted mod list. None for an empty name.
pub fn mod_initial(name: &str) -> Option<char> {
    // TODO: take name.chars().next() with `?`, then uppercase it
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mods_for_client_and_server_doubles_count() {
        assert_eq!(mods_for_client_and_server("21"), Ok(42));
        assert_eq!(mods_for_client_and_server("-4"), Ok(-8));
    }

    #[test]
    fn bad_mod_count_comes_back_as_err() {
        assert!(mods_for_client_and_server("21x").is_err(), "\"21x\" is not a mod count");
        assert!(mods_for_client_and_server("").is_err(), "empty string is not a mod count");
    }

    #[test]
    fn mod_initial_works_on_option() {
        assert_eq!(mod_initial("sodium"), Some('S'));
        assert_eq!(mod_initial(""), None, "empty name has no first letter");
    }
}
