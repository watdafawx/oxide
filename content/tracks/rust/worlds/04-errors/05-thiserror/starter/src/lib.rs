use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    // TODO: give this variant the message "missing key: {0}" (`{0}` is its first field)
    #[error("TODO")]
    Missing(String),

    // TODO: message "port is not a number: {0}"; mark the field so `?` can convert a ParseIntError into it
    #[error("TODO")]
    BadPort(ParseIntError),
}

/// Find `key` in `pairs` and parse its value as a port number.
pub fn read_port(pairs: &[(&str, &str)], key: &str) -> Result<u16, ConfigError> {
    // TODO: find the pair whose key matches (no match -> Missing carrying the key),
    //       then parse the value and let `?` convert a failure
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_key_message_names_the_key() {
        let err = read_port(&[("host", "x")], "port").unwrap_err();
        assert_eq!(err.to_string(), "missing key: port");
    }

    #[test]
    fn bad_port_converts_with_from() {
        let err = read_port(&[("port", "80x")], "port").unwrap_err();
        assert!(matches!(err, ConfigError::BadPort(_)), "expected BadPort, got {err:?}");
        assert_eq!(err.to_string(), "port is not a number: invalid digit found in string");
    }

    #[test]
    fn read_port_finds_a_valid_port() {
        let pairs = [("host", "localhost"), ("port", "8080")];
        assert_eq!(read_port(&pairs, "port").unwrap(), 8080);
    }
}
