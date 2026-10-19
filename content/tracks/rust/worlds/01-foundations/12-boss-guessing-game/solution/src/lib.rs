#[derive(Debug, PartialEq)]
pub enum Outcome {
    TooLow,
    TooHigh,
    Correct,
}

pub struct Game {
    secret: u32,
    attempts: u32,
}

impl Game {
    /// A fresh game with no attempts yet.
    pub fn new(secret: u32) -> Game {
        Game { secret, attempts: 0 }
    }

    /// Count the attempt, then compare `n` with the secret.
    pub fn guess(&mut self, n: u32) -> Outcome {
        self.attempts += 1;
        if n < self.secret {
            Outcome::TooLow
        } else if n > self.secret {
            Outcome::TooHigh
        } else {
            Outcome::Correct
        }
    }

    /// How many guesses were made.
    pub fn attempts(&self) -> u32 {
        self.attempts
    }
}

/// " 42 " -> Ok(42), "abc" -> Err("'abc' is not a number")
pub fn parse_guess(input: &str) -> Result<u32, String> {
    let text = input.trim();
    text.parse::<u32>()
        .map_err(|_| format!("'{text}' is not a number"))
}

/// Run a whole game from scripted input and return what the player would see.
/// Invalid input shows its error but does not count as an attempt.
/// Stops at the first correct guess: "Correct! 3 attempts" ("1 attempt" if just one).
pub fn play(secret: u32, inputs: &[&str]) -> Vec<String> {
    let mut game = Game::new(secret);
    let mut lines = Vec::new();
    for input in inputs {
        match parse_guess(input) {
            Err(message) => lines.push(message),
            Ok(n) => match game.guess(n) {
                Outcome::TooLow => lines.push("Too low".to_string()),
                Outcome::TooHigh => lines.push("Too high".to_string()),
                Outcome::Correct => {
                    let attempts = game.attempts();
                    let s = if attempts == 1 { "" } else { "s" };
                    lines.push(format!("Correct! {attempts} attempt{s}"));
                    break;
                }
            },
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_starts_with_zero_attempts() {
        let game = Game::new(7);
        assert_eq!(game.attempts(), 0);
    }

    #[test]
    fn guess_reports_too_low_and_too_high() {
        let mut game = Game::new(50);
        assert_eq!(game.guess(10), Outcome::TooLow);
        assert_eq!(game.guess(90), Outcome::TooHigh);
    }

    #[test]
    fn guess_reports_correct() {
        let mut game = Game::new(50);
        assert_eq!(game.guess(50), Outcome::Correct);
    }

    #[test]
    fn attempts_are_counted() {
        let mut game = Game::new(50);
        game.guess(1);
        game.guess(2);
        game.guess(50);
        assert_eq!(game.attempts(), 3, "three guesses were made");
    }

    #[test]
    fn parse_guess_accepts_numbers() {
        assert_eq!(parse_guess("42"), Ok(42));
        assert_eq!(parse_guess("  7 \n"), Ok(7), "surrounding whitespace is fine");
    }

    #[test]
    fn parse_guess_rejects_bad_input() {
        assert_eq!(parse_guess("abc"), Err("'abc' is not a number".to_string()));
        assert!(parse_guess("-5").is_err(), "negative numbers are not valid guesses");
        assert!(parse_guess("").is_err(), "empty input is not a number");
    }

    #[test]
    fn play_reports_each_guess() {
        let out = play(7, &["3", "9", "x", "7"]);
        assert_eq!(
            out,
            vec!["Too low", "Too high", "'x' is not a number", "Correct! 3 attempts"],
            "bad input must not count as an attempt"
        );
    }

    #[test]
    fn play_stops_after_correct_guess() {
        let out = play(5, &["5", "1", "2"]);
        assert_eq!(out, vec!["Correct! 1 attempt"], "singular for one attempt, and stop playing");
    }
}
