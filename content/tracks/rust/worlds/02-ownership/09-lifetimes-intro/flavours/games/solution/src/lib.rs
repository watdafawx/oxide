/// A parsed console line. It points into the line it came from: no copying.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Command<'a> {
    pub name: &'a str,
    pub arg: &'a str,
}

impl<'a> Command<'a> {
    /// "kick Steve" -> name "kick", arg "Steve". No space: the arg is empty.
    pub fn parse(line: &'a str) -> Command<'a> {
        let (name, arg) = line.split_once(' ').unwrap_or((line, ""));
        Command { name, arg }
    }
}

/// The longer argument of the two commands (the first one if they are the same length).
/// The answer points into the original lines, so it is valid as long as `'a`.
pub fn longest_arg<'a>(a: Command<'a>, b: Command<'a>) -> &'a str {
    if a.arg.len() >= b.arg.len() { a.arg } else { b.arg }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_arg_picks_longer_argument() {
        let a = Command::parse("kick Bo");
        let b = Command::parse("kick Alexander");
        assert_eq!(longest_arg(a, b), "Alexander");
        assert_eq!(longest_arg(b, a), "Alexander");
    }

    #[test]
    fn longest_arg_tie_keeps_first_command() {
        let first = String::from("kick abc");
        let second = String::from("kick xyz");
        let r = longest_arg(Command::parse(&first), Command::parse(&second));
        assert_eq!(r.as_ptr(), first[5..].as_ptr(), "on a tie the first command wins");
    }

    #[test]
    fn argument_usable_while_console_lines_live() {
        let x = String::from("tp Rustacean");
        let y = String::from("tp Crab");
        let r = longest_arg(Command::parse(&x), Command::parse(&y));
        assert_eq!(r, "Rustacean");
        assert_eq!(x.len() + y.len(), 19);
    }
}
