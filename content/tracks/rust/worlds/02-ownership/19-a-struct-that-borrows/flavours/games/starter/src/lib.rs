/// A parsed console line. It borrows both parts from the line: no copying.
// TODO: this does not build: say how long the borrowed parts live.
// Give the struct a lifetime `'a` and use it on both fields.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Command {
    pub name: &str,
    pub arg: &str,
}

/// "kick Steve" -> name "kick", arg "Steve". No space: the arg is "".
pub fn parse(line: &str) -> Command<'_> {
    // TODO: borrow the part before the first space as the name and the part after it as the arg
    todo!()
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
