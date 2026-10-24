/// The command word: everything before the first space; the whole line if there is no space.
pub fn command_name(line: &str) -> &str {
    match line.find(' ') {
        Some(i) => &line[..i],
        None => line,
    }
}

/// The arguments: everything after the first space; "" if there is no space.
pub fn command_args(line: &str) -> &str {
    match line.find(' ') {
        Some(i) => &line[i + 1..],
        None => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_name_stops_at_first_space() {
        assert_eq!(command_name("kick Steve griefing"), "kick");
        assert_eq!(command_name("list"), "list");
        assert_eq!(command_name(""), "");
        let line = String::from("tp Alex 0 64 0");
        assert_eq!(command_name(&line).as_ptr(), line.as_ptr(), "slice the line, don't copy it");
    }

    #[test]
    fn command_args_are_the_rest() {
        assert_eq!(command_args("kick Steve griefing"), "Steve griefing");
        assert_eq!(command_args("list"), "");
    }
}
