pub enum Command {
    Quit,
    Say(String),
    Move { dx: i32, dy: i32 },
}

/// 90..=100 -> 'A', 80..=89 -> 'B', 70..=79 -> 'C', anything else -> 'F'
pub fn grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F',
    }
}

/// negative, zero, even, odd
pub fn describe_number(n: i32) -> &'static str {
    match n {
        n if n < 0 => "negative",
        0 => "zero",
        n if n % 2 == 0 => "even",
        _ => "odd",
    }
}

/// Move changes the position, Quit and Say leave it alone.
pub fn apply(cmd: &Command, pos: (i32, i32)) -> (i32, i32) {
    match cmd {
        Command::Move { dx, dy } => (pos.0 + dx, pos.1 + dy),
        Command::Quit | Command::Say(_) => pos,
    }
}

/// The text of a Say command, or an empty String for anything else.
pub fn say_text(cmd: &Command) -> String {
    if let Command::Say(text) = cmd {
        text.clone()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grades_use_ranges() {
        assert_eq!(grade(95), 'A');
        assert_eq!(grade(80), 'B');
        assert_eq!(grade(79), 'C');
        assert_eq!(grade(12), 'F');
    }

    #[test]
    fn describe_number_uses_guards() {
        assert_eq!(describe_number(-4), "negative");
        assert_eq!(describe_number(0), "zero");
        assert_eq!(describe_number(8), "even");
        assert_eq!(describe_number(7), "odd");
    }

    #[test]
    fn apply_moves_the_position() {
        assert_eq!(apply(&Command::Move { dx: 2, dy: -1 }, (5, 5)), (7, 4));
        assert_eq!(apply(&Command::Quit, (5, 5)), (5, 5), "Quit does not move");
        assert_eq!(apply(&Command::Say("hi".to_string()), (1, 1)), (1, 1));
    }

    #[test]
    fn say_text_uses_if_let() {
        assert_eq!(say_text(&Command::Say("hello".to_string())), "hello");
        assert_eq!(say_text(&Command::Quit), "");
    }
}
