use own_text_adventure::*;
use std::io::{self, BufRead, Write};
use std::panic;

fn main() {
    println!("== Server console ==");
    println!("Commands: list, tp <name> <x> <y> <z>, give <name> <item>, kick <name>, quit.");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        // `run` takes a whole script, so replay the history and print the new lines.
        let mut history: Vec<String> = vec!["list".to_string()];
        let mut shown = 0;
        let stdin = io::stdin();
        let mut first = true;
        loop {
            if !first {
                print!("> ");
                io::stdout().flush().ok();
                let mut line = String::new();
                if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 || line.trim() == "quit" {
                    println!("Bye!");
                    break;
                }
                if line.trim().is_empty() {
                    continue;
                }
                history.push(line.trim().to_string());
            }
            first = false;
            let script: Vec<&str> = history.iter().map(String::as_str).collect();
            let output = run(&script);
            for out in &output[shown.min(output.len())..] {
                println!("{out}");
            }
            shown = output.len();
        }
    });
    if result.is_err() {
        println!("Finish `list`, `tp`, `give`, `kick` and `run` to use the console.");
    }
}
