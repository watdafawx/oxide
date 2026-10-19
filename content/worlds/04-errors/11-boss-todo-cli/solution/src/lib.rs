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

#[derive(Debug, Default)]
pub struct Store {
    todos: Vec<Todo>,
}

/// One todo as a file line: `id|0 or 1|text`. Given: you don't need to change it.
fn format_line(todo: &Todo) -> String {
    format!("{}|{}|{}", todo.id, u8::from(todo.done), todo.text)
}

/// Read one file line back into a Todo. Given: you don't need to change it.
fn parse_line(line: &str) -> anyhow::Result<Todo> {
    let mut parts = line.splitn(3, '|');
    let (Some(id), Some(done), Some(text)) = (parts.next(), parts.next(), parts.next()) else {
        bail!("expected id|done|text, got {line:?}");
    };
    Ok(Todo {
        id: id.parse().context("bad id")?,
        done: done == "1",
        text: text.to_string(),
    })
}

impl Store {
    /// An empty store.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn todos(&self) -> &[Todo] {
        &self.todos
    }

    /// Add a todo and return its id. Ids start at 1 and go up (highest existing id + 1).
    pub fn add(&mut self, text: &str) -> u32 {
        let id = self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.todos.push(Todo { id, text: text.to_string(), done: false });
        id
    }

    /// Flip the `done` flag of todo `id`. Unknown id -> TodoError::NotFound(id).
    pub fn toggle(&mut self, id: u32) -> Result<(), TodoError> {
        let todo = self
            .todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::NotFound(id))?;
        todo.done = !todo.done;
        Ok(())
    }

    /// Read a store from `path`. A file that does not exist yet is an empty store.
    pub fn load(path: &Path) -> anyhow::Result<Store> {
        let data = match std::fs::read_to_string(path) {
            Ok(data) => data,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Store::new()),
            Err(e) => return Err(e).with_context(|| format!("reading {}", path.display())),
        };
        let mut todos = Vec::new();
        for (i, line) in data.lines().enumerate() {
            todos.push(parse_line(line).with_context(|| format!("line {}", i + 1))?);
        }
        Ok(Store { todos })
    }

    /// Write the store to `path`, one `format_line` per todo.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let lines: Vec<String> = self.todos.iter().map(format_line).collect();
        std::fs::write(path, lines.join("\n"))
            .with_context(|| format!("writing {}", path.display()))
    }
}

/// Turn "add buy milk" / "done 3" / "list" into a Command.
pub fn parse_command(input: &str) -> Result<Command, TodoError> {
    let input = input.trim();
    let bad = || TodoError::BadCommand(input.to_string());
    let (verb, rest) = input.split_once(' ').unwrap_or((input, ""));
    let rest = rest.trim();
    match verb {
        "list" => Ok(Command::List),
        "add" if !rest.is_empty() => Ok(Command::Add(rest.to_string())),
        "done" => rest.parse().map(Command::Done).map_err(|_| bad()),
        _ => Err(bad()),
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
            if store.todos().is_empty() {
                return Ok("nothing to do".to_string());
            }
            let lines: Vec<String> = store
                .todos()
                .iter()
                .map(|t| format!("[{}] {} {}", if t.done { 'x' } else { ' ' }, t.id, t.text))
                .collect();
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
