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

/// The longer argument of the two commands (the first one's if they are the same length).
pub fn longer_arg<'a>(a: Command<'a>, b: Command<'a>) -> &'a str {
    if a.arg.len() >= b.arg.len() { a.arg } else { b.arg }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longer_arg_picks_longer_argument() {
        let a = parse("kick Bo");
        let b = parse("kick Alexander");
        assert_eq!(longer_arg(a, b), "Alexander");
        assert_eq!(longer_arg(b, a), "Alexander");
    }

    #[test]
    fn longer_arg_tie_keeps_first_command() {
        let first = String::from("kick abc");
        let second = String::from("kick xyz");
        let r = longer_arg(parse(&first), parse(&second));
        assert_eq!(r.as_ptr(), first[5..].as_ptr(), "on a tie the first command wins");
    }
}
