use ptr_boss_tree_editor::*;
use std::io::{self, BufRead, Write};
use std::panic;

fn main() {
    println!("== Tree editor ==");
    println!("Commands: show, add <parent> <text>, rm <id>, mv <id> <new parent>, path <id>, quit.");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let mut doc = Doc::new("Notes");
        doc.add(ROOT, "Ideas");
        print!("{}", doc.render());
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush().ok();
            let mut line = String::new();
            if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 || line.trim() == "quit" {
                println!("Bye!");
                break;
            }
            let words: Vec<&str> = line.split_whitespace().collect();
            match words.as_slice() {
                [] => {}
                ["show"] => print!("{}", doc.render()),
                ["add", parent, name @ ..] if !name.is_empty() => match parent.parse() {
                    Ok(parent) => match doc.add(parent, &name.join(" ")) {
                        Some(id) => println!("added {id}"),
                        None => println!("no node {parent}"),
                    },
                    Err(_) => println!("'{parent}' is not an id"),
                },
                ["rm", id] => match id.parse() {
                    Ok(id) => println!("{}", if doc.remove(id) { "removed" } else { "can't remove that" }),
                    Err(_) => println!("'{id}' is not an id"),
                },
                ["mv", id, to] => match (id.parse(), to.parse()) {
                    (Ok(id), Ok(to)) => println!("{}", if doc.move_node(id, to) { "moved" } else { "can't move that" }),
                    _ => println!("ids are numbers"),
                },
                ["path", id] => match id.parse() {
                    Ok(id) => match doc.path(id).as_slice() {
                        [] => println!("no node {id}"),
                        path => println!("{}", path.join(" > ")),
                    },
                    Err(_) => println!("'{id}' is not an id"),
                },
                _ => println!("commands: show, add <parent> <text>, rm <id>, mv <id> <new parent>, path <id>, quit"),
            }
        }
    });
    if result.is_err() {
        println!("Finish `Doc::new`, `add`, `path`, `render`, `remove` and `move_node` to edit the tree.");
    }
}
