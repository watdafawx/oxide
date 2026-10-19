use found_guessing_game::*;
use std::io::{self, BufRead, Write};
use std::panic;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("== Guessing game ==");
    println!("I picked a number from 1 to 100. Type a guess, or `quit`.");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
        let mut game = Game::new(nanos % 100 + 1);
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().ok();
            let mut line = String::new();
            if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 || line.trim() == "quit" {
                println!("Bye!");
                break;
            }
            match parse_guess(&line) {
                Err(message) => println!("{message}"),
                Ok(n) => match game.guess(n) {
                    Outcome::TooLow => println!("Too low"),
                    Outcome::TooHigh => println!("Too high"),
                    Outcome::Correct => {
                        println!("Correct! {} attempts", game.attempts());
                        break;
                    }
                },
            }
        }
    });
    if result.is_err() {
        println!("Finish `Game::new`, `guess`, `attempts` and `parse_guess` to play.");
    }
}

