use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    // TODO: change the message to "missing key: {0}"
    #[error("TODO")]
    Missing(String),

    // TODO: message "port is not a number: {0}" and put #[from] on the field
    #[error("TODO")]
    BadPort(ParseIntError),
}

/// Find `key` in `pairs` and parse its value as a port number.
pub fn read_port(pairs: &[(&str, &str)], key: &str) -> Result<u16, ConfigError> {
    // TODO: find the pair whose key matches (ok_or_else -> ConfigError::Missing),
    //       then parse the value with `?` (needs #[from] on BadPort)
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
