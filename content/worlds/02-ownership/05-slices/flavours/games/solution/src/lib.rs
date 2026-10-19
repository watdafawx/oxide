/// The command word: everything before the first space; the whole line if there is no space.
pub fn command_name(line: &str) -> &str {
    match line.find(' ') {
        Some(i) => &line[..i],
        None => line,
    }
}

/// Drops the first and last line of a log dump (header and footer). Empty if there are 2 or fewer.
pub fn trim_edges<'a>(lines: &'a [&'a str]) -> &'a [&'a str] {
    if lines.len() <= 2 {
        &[]
    } else {
        &lines[1..lines.len() - 1]
    }
}

/// The last 3 chat lines, or everything if there are fewer than 3.
pub fn recent_lines(log: &[String]) -> &[String] {
    let start = log.len().saturating_sub(3);
    &log[start..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_stops_at_space() {
        assert_eq!(command_name("kick Steve griefing"), "kick");
        assert_eq!(command_name("list"), "list");
        assert_eq!(command_name(""), "");
    }

    #[test]
    fn middle_drops_both_ends() {
        assert_eq!(
            trim_edges(&["== log ==", "steve joined", "alex left", "== end =="]),
            &["steve joined", "alex left"]
        );
        assert_eq!(trim_edges(&["== log ==", "== end =="]), &[] as &[&str]);
        assert_eq!(trim_edges(&[]), &[] as &[&str]);
    }

    #[test]
    fn last_three_takes_the_tail() {
        let log: Vec<String> = ["a", "b", "c", "d", "e"].iter().map(|s| s.to_string()).collect();
        assert_eq!(recent_lines(&log), &["c", "d", "e"]);
        let short = vec!["x".to_string(), "y".to_string()];
        assert_eq!(recent_lines(&short), &["x", "y"]);
    }
}
