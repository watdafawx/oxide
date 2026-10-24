/// A parsed console line. It borrows both parts from the line: no copying.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Command<'a> {
    pub name: &'a str,
    pub arg: &'a str,
}

/// "kick Steve" -> name "kick", arg "Steve". No space: the arg is "".
pub fn parse(line: &str) -> Command<'_> {
    match line.find(' ') {
        Some(i) => Command { name: &line[..i], arg: &line[i + 1..] },
        None => Command { name: line, arg: "" },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_borrows_the_line() {
        let line = String::from("kick Steve");
        let cmd = parse(&line);
        assert_eq!(cmd, Command { name: "kick", arg: "Steve" });
        assert_eq!(cmd.name.as_ptr(), line.as_ptr(), "the name must borrow the line, not copy it");
        assert_eq!(parse("list"), Command { name: "list", arg: "" });
    }
}
