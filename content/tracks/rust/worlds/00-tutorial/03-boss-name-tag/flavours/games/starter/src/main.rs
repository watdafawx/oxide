use name_tag::*;
use std::panic;

fn main() {
    println!("== Name tags ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        for (name, level) in [("ferris", 3), ("mia", 12), ("oxide", 99)] {
            println!("{}", player_tag(name, level));
        }
    });
    if result.is_err() {
        println!("Finish `loud_name` and `player_tag` to print the tags.");
    }
}
