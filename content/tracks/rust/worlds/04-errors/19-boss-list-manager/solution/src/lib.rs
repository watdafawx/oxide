use std::fs;
use std::io;
use std::path::Path;

use anyhow::{Context, bail};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub done: bool,
}

#[derive(Debug, PartialEq, Error)]
pub enum TodoError {
    #[error("no todo with id {0}")]
    NotFound(u32),
    #[error("unknown command: {0}")]
    BadCommand(String),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(String),
    Done(u32),
    List,
}

#[derive(Debug)]
pub struct Store {
    todos: Vec<Todo>,
}

/// One todo as a file line: `id|0 or 1|text`. Given: you don't need to change it.
fn format_line(todo: &Todo) -> String {
    let done = if todo.done { 1 } else { 0 };
    format!("{}|{}|{}", todo.id, done, todo.text)
}

/// Read one file line back into a Todo. Given: you don't need to change it.
fn parse_line(line: &str) -> anyhow::Result<Todo> {
    let parts: Vec<&str> = line.splitn(3, '|').collect();
    if parts.len() != 3 {
        bail!("expected id|done|text, got {line:?}");
    }
    Ok(Todo {
        id: parts[0].parse().context("bad id")?,
        done: parts[1] == "1",
        text: parts[2].to_string(),
    })
}

/// Trim the input and cut off the first word: "done 3" -> ("done", "3"), "list" -> ("list", "").
/// Given: you don't need to change it.
fn split_verb(input: &str) -> (&str, &str) {
    let input = input.trim();
    match input.split_once(' ') {
        Some((verb, rest)) => (verb, rest.trim()),
        None => (input, ""),
    }
}

impl Store {
    /// An empty store.
    pub fn new() -> Self {
        Store { todos: Vec::new() }
    }

    pub fn todos(&self) -> &[Todo] {
        &self.todos
    }

    /// Add a todo and return its id. Ids start at 1 and go up (highest existing id + 1).
    pub fn add(&mut self, text: &str) -> u32 {
        let mut highest = 0;
        for todo in &self.todos {
            if todo.id > highest {
                highest = todo.id;
            }
        }
        let id = highest + 1;
        self.todos.push(Todo { id, text: text.to_string(), done: false });
        id
    }

    /// Flip the `done` flag of todo `id`. Unknown id -> TodoError::NotFound(id).
    pub fn toggle(&mut self, id: u32) -> Result<(), TodoError> {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.done = !todo.done;
                return Ok(());
            }
        }
        Err(TodoError::NotFound(id))
    }

    /// Read a store from `path`. A file that does not exist yet is an empty store.
    pub fn load(path: &Path) -> anyhow::Result<Store> {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Store::new()),
            Err(e) => return Err(e).with_context(|| format!("could not read {}", path.display())),
        };
        let mut store = Store::new();
        for (i, line) in text.lines().enumerate() {
            store.todos.push(parse_line(line).with_context(|| format!("line {}", i + 1))?);
        }
        Ok(store)
    }

    /// Write the store to `path`, one `format_line` per todo.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let mut text = String::new();
        for todo in &self.todos {
            text.push_str(&format_line(todo));
            text.push('\n');
        }
        fs::write(path, text).with_context(|| format!("could not write {}", path.display()))
    }
}

/// Given: `Store::default()` works too. Clippy, Rust's style checker, expects this next to a `new`.
impl Default for Store {
    fn default() -> Self {
        Store::new()
    }
}

/// Turn "add buy milk" / "done 3" / "list" into a Command.
pub fn parse_command(input: &str) -> Result<Command, TodoError> {
    let (verb, rest) = split_verb(input);
    let bad = TodoError::BadCommand(input.trim().to_string());
    match verb {
        "add" => {
            if rest.is_empty() {
                Err(bad)
            } else {
                Ok(Command::Add(rest.to_string()))
            }
        }
        "done" => match rest.parse::<u32>() {
            Ok(id) => Ok(Command::Done(id)),
            Err(_) => Err(bad),
        },
        "list" => Ok(Command::List),
        _ => Err(bad),
    }
}

/// Run a command. Add -> "added <id>", Done -> "done <id>",
/// List -> one line per todo like "[x] 2 eggs" / "[ ] 1 milk", or "nothing to do".
pub fn run(store: &mut Store, cmd: Command) -> anyhow::Result<String> {
    match cmd {
        Command::Add(text) => Ok(format!("added {}", store.add(&text))),
        Command::Done(id) => {
            store.toggle(id)?;
            Ok(format!("done {id}"))
        }
        Command::List => {
            if store.todos.is_empty() {
                return Ok("nothing to do".to_string());
            }
            let mut lines = Vec::new();
            for todo in &store.todos {
                let mark = if todo.done { 'x' } else { ' ' };
                lines.push(format!("[{}] {} {}", mark, todo.id, todo.text));
            }
            Ok(lines.join("\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxide_todo_{}_{}.txt", std::process::id(), name))
    }

    #[test]
    fn new_store_is_empty() {
        assert_eq!(Store::new().todos().len(), 0);
    }

    #[test]
    fn add_assigns_increasing_ids() {
        let mut store = Store::new();
        assert_eq!(store.add("milk"), 1);
        assert_eq!(store.add("eggs"), 2);
        let todos = store.todos();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[1], Todo { id: 2, text: "eggs".to_string(), done: false });
    }

    #[test]
    fn toggle_flips_done_and_reports_missing_ids() {
        let mut store = Store::new();
        let id = store.add("milk");
        assert_eq!(store.toggle(id), Ok(()));
        assert!(store.todos()[0].done, "first toggle marks it done");
        store.toggle(id).unwrap();
        assert!(!store.todos()[0].done, "second toggle undoes it");
        assert_eq!(store.toggle(99), Err(TodoError::NotFound(99)));
    }

    #[test]
    fn parse_command_understands_all_three() {
        assert_eq!(parse_command("add buy milk"), Ok(Command::Add("buy milk".to_string())));
        assert_eq!(parse_command("done 3"), Ok(Command::Done(3)));
        assert_eq!(parse_command("  list "), Ok(Command::List));
    }

    #[test]
    fn parse_command_rejects_nonsense() {
        for bad in ["", "add", "done abc", "dance"] {
            assert_eq!(
                parse_command(bad),
                Err(TodoError::BadCommand(bad.trim().to_string())),
                "input {bad:?} should be a BadCommand"
            );
        }
    }

    #[test]
    fn save_then_load_round_trips() {
        let path = temp_path("roundtrip");
        let mut store = Store::new();
        store.add("milk");
        store.add("a | pipe in text");
        store.toggle(2).unwrap();
        store.save(&path).unwrap();

        let loaded = Store::load(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        assert_eq!(loaded.todos(), store.todos());
    }

    #[test]
    fn load_missing_file_gives_empty_store() {
        let path = temp_path("does_not_exist");
        let _ = std::fs::remove_file(&path);
        let store = Store::load(&path).expect("a missing file is not an error");
        assert_eq!(store.todos().len(), 0);
    }

    #[test]
    fn run_executes_commands() {
        let mut store = Store::new();
        assert_eq!(run(&mut store, Command::List).unwrap(), "nothing to do");
        assert_eq!(run(&mut store, Command::Add("milk".to_string())).unwrap(), "added 1");
        run(&mut store, Command::Add("eggs".to_string())).unwrap();
        run(&mut store, Command::Done(2)).unwrap();
        assert_eq!(run(&mut store, Command::List).unwrap(), "[ ] 1 milk\n[x] 2 eggs");

        let err = run(&mut store, Command::Done(9)).unwrap_err();
        assert_eq!(err.downcast_ref::<TodoError>(), Some(&TodoError::NotFound(9)));
    }
}
