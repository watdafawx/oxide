use found_boss_crafting_calculator::*;
use std::io::{self, BufRead, Write};
use std::panic;

fn main() {
    println!("== Crafting calculator ==");
    println!("Teach it recipes like `stick x4 = plank x2`, ask `need 8 stick`, or `quit`.");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        // `run` takes a whole script, so replay the history and print the new lines.
        let mut history: Vec<String> = Vec::new();
        let mut shown = 0;
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().ok();
            let mut line = String::new();
            if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
                break;
            }
            if line.trim().is_empty() {
                continue;
            }
            history.push(line.trim().to_string());
            let script: Vec<&str> = history.iter().map(String::as_str).collect();
            let output = run(&script);
            for out in &output[shown.min(output.len())..] {
                println!("{out}");
            }
            shown = output.len();
            if line.trim() == "quit" {
                break;
            }
        }
    });
    if result.is_err() {
        println!("Finish `Calculator` and `run` to use the calculator.");
    }
}
