/// The command word: everything before the first space; the whole line if there is no space.
pub fn command_name(line: &str) -> &str {
    // TODO: find the first space and slice up to it
    todo!()
}

/// Drops the first and last line of a log dump (header and footer). Empty if there are 2 or fewer.
pub fn trim_edges<'a>(lines: &'a [&'a str]) -> &'a [&'a str] {
    // TODO: slice from index 1 to len - 1, but guard short inputs
    todo!()
}

/// The last 3 chat lines, or everything if there are fewer than 3.
pub fn recent_lines(log: &[String]) -> &[String] {
    // TODO: compute the start with saturating_sub, then slice to the end
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_name_stops_at_first_space() {
        assert_eq!(command_name("kick Steve griefing"), "kick");
        assert_eq!(command_name("list"), "list");
        assert_eq!(command_name(""), "");
    }

    #[test]
    fn trim_edges_drops_header_and_footer() {
        assert_eq!(
            trim_edges(&["== log ==", "steve joined", "alex left", "== end =="]),
            &["steve joined", "alex left"]
        );
        assert_eq!(trim_edges(&["== log ==", "== end =="]), &[] as &[&str]);
        assert_eq!(trim_edges(&[]), &[] as &[&str]);
    }

    #[test]
    fn recent_lines_keeps_last_three() {
        let log: Vec<String> = ["a", "b", "c", "d", "e"].iter().map(|s| s.to_string()).collect();
        assert_eq!(recent_lines(&log), &["c", "d", "e"]);
        let short = vec!["x".to_string(), "y".to_string()];
        assert_eq!(recent_lines(&short), &["x", "y"]);
    }
}
