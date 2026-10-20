use err_boss_modlist::*;
use std::io::{self, BufRead, Write};
use std::panic;
use std::path::Path;

fn main() {
    println!("== Mod list manager ==");
    println!("Commands: add <name>, toggle <id>, list, quit. Saved to mods.txt.");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let path = Path::new("mods.txt");
        let mut list = match ModList::load(path) {
            Ok(loaded) => loaded,
            Err(e) => {
                println!("could not load {}: {e:#}", path.display());
                ModList::new()
            }
        };
        let stdin = io::stdin();
        loop {
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
            match parse_command(&line) {
                Err(e) => println!("error: {e}"),
                Ok(cmd) => match run(&mut list, cmd) {
                    Ok(reply) => println!("{reply}"),
                    Err(e) => println!("error: {e:#}"),
                },
            }
            if let Err(e) = list.save(path) {
                println!("could not save: {e:#}");
            }
        }
    });
    if result.is_err() {
        println!("Finish `add`, `toggle`, `load`, `save`, `parse_command` and `run` to use the mod manager.");
    }
}
