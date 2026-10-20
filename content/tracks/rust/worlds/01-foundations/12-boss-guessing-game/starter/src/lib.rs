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
        // TODO: build a Game holding the secret and no attempts yet
        todo!()
    }

    /// Count the attempt, then compare `n` with the secret.
    pub fn guess(&mut self, n: u32) -> Outcome {
        // TODO: count this attempt, then compare n with self.secret and return the matching Outcome
        todo!()
    }

    /// How many guesses were made.
    pub fn attempts(&self) -> u32 {
        // TODO: return the counter
        todo!()
    }
}

/// " 42 " -> Ok(42), "abc" -> Err("'abc' is not a number")
pub fn parse_guess(input: &str) -> Result<u32, String> {
    // TODO: trim the input, parse it as a u32, and turn a parse failure into your message
    todo!()
}

/// Run a whole game from scripted input and return what the player would see.
/// Invalid input shows its error but does not count as an attempt.
/// Stops at the first correct guess: "Correct! 3 attempts" ("1 attempt" if just one).
pub fn play(secret: u32, inputs: &[&str]) -> Vec<String> {
    // TODO: start a game and an empty message list; for each input, parse it, and push a message for the error or the guess outcome
    todo!()
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
