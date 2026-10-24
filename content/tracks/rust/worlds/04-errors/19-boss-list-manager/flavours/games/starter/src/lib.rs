use std::fs;
use std::io;
use std::path::Path;

use anyhow::{Context, bail};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct ModEntry {
    pub id: u32,
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, PartialEq, Error)]
pub enum ModError {
    #[error("no mod with id {0}")]
    NotFound(u32),
    #[error("unknown command: {0}")]
    BadCommand(String),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(String),
    Toggle(u32),
    List,
}

#[derive(Debug)]
pub struct ModList {
    mods: Vec<ModEntry>,
}

/// One mod as a file line: `id|0 or 1|name`. Given: you don't need to change it.
fn format_line(entry: &ModEntry) -> String {
    let enabled = if entry.enabled { 1 } else { 0 };
    format!("{}|{}|{}", entry.id, enabled, entry.name)
}

/// Read one file line back into a ModEntry. Given: you don't need to change it.
fn parse_line(line: &str) -> anyhow::Result<ModEntry> {
    let parts: Vec<&str> = line.splitn(3, '|').collect();
    if parts.len() != 3 {
        bail!("expected id|enabled|name, got {line:?}");
    }
    Ok(ModEntry {
        id: parts[0].parse().context("bad id")?,
        enabled: parts[1] == "1",
        name: parts[2].to_string(),
    })
}

/// Trim the input and cut off the first word: "toggle 3" -> ("toggle", "3"), "list" -> ("list", "").
/// Given: you don't need to change it.
fn split_verb(input: &str) -> (&str, &str) {
    let input = input.trim();
    match input.split_once(' ') {
        Some((verb, rest)) => (verb, rest.trim()),
        None => (input, ""),
    }
}

impl ModList {
    /// An empty mod list.
    pub fn new() -> Self {
        // TODO: a ModList with an empty list of mods
        todo!()
    }

    pub fn mods(&self) -> &[ModEntry] {
        &self.mods
    }

    /// Add a mod (enabled) and return its id. Ids start at 1 and go up (highest existing id + 1).
    pub fn add(&mut self, name: &str) -> u32 {
        // TODO: find the highest id so far (0 when empty), store a new mod with the next id, enabled, and return that id
        todo!()
    }

    /// Flip the `enabled` flag of mod `id`. Unknown id -> ModError::NotFound(id).
    pub fn toggle(&mut self, id: u32) -> Result<(), ModError> {
        // TODO: loop with `for entry in &mut self.mods`; flip `enabled` on the one with this id and return Ok,
        //       or return NotFound after the loop
        todo!()
    }

    /// Read a mod list from `path`. A file that does not exist yet is an empty mod list.
    pub fn load(path: &Path) -> anyhow::Result<ModList> {
        // TODO: read the file. NotFound -> an empty mod list; any other error gets the context "could not read <path>".
        //       Then parse every line with parse_line, adding the context "line <n>" (counting from 1, enumerate helps) to a failure
        todo!()
    }

    /// Write the mod list to `path`, one `format_line` per mod.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        // TODO: build the text, one format_line per mod, each followed by a line break,
        //       then write it; a write error gets the context "could not write <path>"
        todo!()
    }
}

/// Given: `ModList::default()` works too. Clippy, Rust's style checker, expects this next to a `new`.
impl Default for ModList {
    fn default() -> Self {
        ModList::new()
    }
}

/// Turn "add sodium" / "toggle 3" / "list" into a Command.
pub fn parse_command(input: &str) -> Result<Command, ModError> {
    // TODO: split off the verb with split_verb. "add" needs a non-empty name, "toggle" needs a number, "list" is List;
    //       anything else (or a bad add/toggle) is BadCommand carrying the trimmed input
    todo!()
}

/// Run a command. Add -> "added <id>", Toggle -> "toggled <id>",
/// List -> one line per mod like "[x] 1 sodium" (enabled) / "[ ] 2 lithium" (disabled), or "no mods yet".
pub fn run(list: &mut ModList, cmd: Command) -> anyhow::Result<String> {
    // TODO: match the command; let `?` turn a ModError into an anyhow::Error.
    //       For List, put each mod's line in a Vec<String> and join them with `join("\n")`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxide_modlist_{}_{}.txt", std::process::id(), name))
    }

    #[test]
    fn new_mod_list_is_empty() {
        assert_eq!(ModList::new().mods().len(), 0);
    }

    #[test]
    fn add_assigns_increasing_mod_ids() {
        let mut list = ModList::new();
        assert_eq!(list.add("sodium"), 1);
        assert_eq!(list.add("lithium"), 2);
        let mods = list.mods();
        assert_eq!(mods.len(), 2);
        assert_eq!(mods[1], ModEntry { id: 2, name: "lithium".to_string(), enabled: true });
    }

    #[test]
    fn toggle_flips_enabled_and_reports_missing_ids() {
        let mut list = ModList::new();
        let id = list.add("sodium");
        assert_eq!(list.toggle(id), Ok(()));
        assert!(!list.mods()[0].enabled, "first toggle disables it");
        list.toggle(id).unwrap();
        assert!(list.mods()[0].enabled, "second toggle enables it again");
        assert_eq!(list.toggle(99), Err(ModError::NotFound(99)));
    }

    #[test]
    fn parse_command_reads_add_toggle_list() {
        assert_eq!(parse_command("add iris shaders"), Ok(Command::Add("iris shaders".to_string())));
        assert_eq!(parse_command("toggle 3"), Ok(Command::Toggle(3)));
        assert_eq!(parse_command("  list "), Ok(Command::List));
    }

    #[test]
    fn parse_command_rejects_bad_input() {
        for bad in ["", "add", "toggle abc", "dance"] {
            assert_eq!(
                parse_command(bad),
                Err(ModError::BadCommand(bad.trim().to_string())),
                "input {bad:?} should be a BadCommand"
            );
        }
    }

    #[test]
    fn mod_list_save_and_load_round_trip() {
        let path = temp_path("roundtrip");
        let mut list = ModList::new();
        list.add("sodium");
        list.add("a | pipe in name");
        list.toggle(2).unwrap();
        list.save(&path).unwrap();

        let loaded = ModList::load(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        assert_eq!(loaded.mods(), list.mods());
    }

    #[test]
    fn load_missing_mod_list_gives_empty_list() {
        let path = temp_path("does_not_exist");
        let _ = std::fs::remove_file(&path);
        let list = ModList::load(&path).expect("a missing file is not an error");
        assert_eq!(list.mods().len(), 0);
    }

    #[test]
    fn run_executes_mod_commands() {
        let mut list = ModList::new();
        assert_eq!(run(&mut list, Command::List).unwrap(), "no mods yet");
        assert_eq!(run(&mut list, Command::Add("sodium".to_string())).unwrap(), "added 1");
        run(&mut list, Command::Add("lithium".to_string())).unwrap();
        run(&mut list, Command::Toggle(2)).unwrap();
        assert_eq!(run(&mut list, Command::List).unwrap(), "[x] 1 sodium\n[ ] 2 lithium");

        let err = run(&mut list, Command::Toggle(9)).unwrap_err();
        assert_eq!(err.downcast_ref::<ModError>(), Some(&ModError::NotFound(9)));
    }
}
