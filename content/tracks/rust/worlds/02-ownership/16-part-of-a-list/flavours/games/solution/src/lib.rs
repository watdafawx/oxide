/// Drops the first and last line of a log dump (header and footer). Empty if there are 2 or fewer.
pub fn trim_edges(lines: &[String]) -> &[String] {
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

    fn lines(texts: &[&str]) -> Vec<String> {
        let mut out = Vec::new();
        for t in texts {
            out.push(t.to_string());
        }
        out
    }

    #[test]
    fn trim_edges_drops_header_and_footer() {
        let dump = lines(&["== log ==", "steve joined", "alex left", "== end =="]);
        assert_eq!(trim_edges(&dump), &["steve joined", "alex left"]);
        assert_eq!(trim_edges(&lines(&["== log ==", "== end =="])), &[] as &[String]);
        assert_eq!(trim_edges(&[]), &[] as &[String]);
    }

    #[test]
    fn recent_lines_keeps_last_three() {
        let log = lines(&["a", "b", "c", "d", "e"]);
        assert_eq!(recent_lines(&log), &["c", "d", "e"]);
        assert_eq!(recent_lines(&lines(&["x", "y"])), &["x", "y"]);
    }
}
