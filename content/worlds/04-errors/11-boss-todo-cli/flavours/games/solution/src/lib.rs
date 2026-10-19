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

#[derive(Debug, Default)]
pub struct ModList {
    mods: Vec<ModEntry>,
}

/// One mod as a file line: `id|0 or 1|name`. Given: you don't need to change it.
fn format_line(entry: &ModEntry) -> String {
    format!("{}|{}|{}", entry.id, u8::from(entry.enabled), entry.name)
}

/// Read one file line back into a ModEntry. Given: you don't need to change it.
fn parse_line(line: &str) -> anyhow::Result<ModEntry> {
    let mut parts = line.splitn(3, '|');
    let (Some(id), Some(enabled), Some(name)) = (parts.next(), parts.next(), parts.next()) else {
        bail!("expected id|enabled|name, got {line:?}");
    };
    Ok(ModEntry {
        id: id.parse().context("bad id")?,
        enabled: enabled == "1",
        name: name.to_string(),
    })
}

impl ModList {
    /// An empty mod list.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mods(&self) -> &[ModEntry] {
        &self.mods
    }

    /// Add a mod (enabled) and return its id. Ids start at 1 and go up (highest existing id + 1).
    pub fn add(&mut self, name: &str) -> u32 {
        let id = self.mods.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        self.mods.push(ModEntry { id, name: name.to_string(), enabled: true });
        id
    }

    /// Flip the `enabled` flag of mod `id`. Unknown id -> ModError::NotFound(id).
    pub fn toggle(&mut self, id: u32) -> Result<(), ModError> {
        let entry = self
            .mods
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(ModError::NotFound(id))?;
        entry.enabled = !entry.enabled;
        Ok(())
    }

    /// Read a mod list from `path`. A file that does not exist yet is an empty list.
    pub fn load(path: &Path) -> anyhow::Result<ModList> {
        let data = match std::fs::read_to_string(path) {
            Ok(data) => data,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(ModList::new()),
            Err(e) => return Err(e).with_context(|| format!("reading {}", path.display())),
        };
        let mut mods = Vec::new();
        for (i, line) in data.lines().enumerate() {
            mods.push(parse_line(line).with_context(|| format!("line {}", i + 1))?);
        }
        Ok(ModList { mods })
    }

    /// Write the mod list to `path`, one `format_line` per mod.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let lines: Vec<String> = self.mods.iter().map(format_line).collect();
        std::fs::write(path, lines.join("\n"))
            .with_context(|| format!("writing {}", path.display()))
    }
}

/// Turn "add sodium" / "toggle 3" / "list" into a Command.
pub fn parse_command(input: &str) -> Result<Command, ModError> {
    let input = input.trim();
    let bad = || ModError::BadCommand(input.to_string());
    let (verb, rest) = input.split_once(' ').unwrap_or((input, ""));
    let rest = rest.trim();
    match verb {
        "list" => Ok(Command::List),
        "add" if !rest.is_empty() => Ok(Command::Add(rest.to_string())),
        "toggle" => rest.parse().map(Command::Toggle).map_err(|_| bad()),
        _ => Err(bad()),
    }
}

/// Run a command. Add -> "added <id>", Toggle -> "toggled <id>",
/// List -> one line per mod like "[x] 1 sodium" (enabled) / "[ ] 2 lithium" (disabled), or "no mods yet".
pub fn run(list: &mut ModList, cmd: Command) -> anyhow::Result<String> {
    match cmd {
        Command::Add(name) => Ok(format!("added {}", list.add(&name))),
        Command::Toggle(id) => {
            list.toggle(id)?;
            Ok(format!("toggled {id}"))
        }
        Command::List => {
            if list.mods().is_empty() {
                return Ok("no mods yet".to_string());
            }
            let lines: Vec<String> = list
                .mods()
                .iter()
                .map(|m| format!("[{}] {} {}", if m.enabled { 'x' } else { ' ' }, m.id, m.name))
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
        std::env::temp_dir().join(format!("oxide_modlist_{}_{}.txt", std::process::id(), name))
    }

    #[test]
    fn new_store_is_empty() {
        assert_eq!(ModList::new().mods().len(), 0);
    }

    #[test]
    fn add_assigns_increasing_ids() {
        let mut list = ModList::new();
        assert_eq!(list.add("sodium"), 1);
        assert_eq!(list.add("lithium"), 2);
        let mods = list.mods();
        assert_eq!(mods.len(), 2);
        assert_eq!(mods[1], ModEntry { id: 2, name: "lithium".to_string(), enabled: true });
    }

    #[test]
    fn toggle_flips_done_and_reports_missing_ids() {
        let mut list = ModList::new();
        let id = list.add("sodium");
        assert_eq!(list.toggle(id), Ok(()));
        assert!(!list.mods()[0].enabled, "first toggle disables it");
        list.toggle(id).unwrap();
        assert!(list.mods()[0].enabled, "second toggle enables it again");
        assert_eq!(list.toggle(99), Err(ModError::NotFound(99)));
    }

    #[test]
    fn parse_command_understands_all_three() {
        assert_eq!(parse_command("add iris shaders"), Ok(Command::Add("iris shaders".to_string())));
        assert_eq!(parse_command("toggle 3"), Ok(Command::Toggle(3)));
        assert_eq!(parse_command("  list "), Ok(Command::List));
    }

    #[test]
    fn parse_command_rejects_nonsense() {
        for bad in ["", "add", "toggle abc", "dance"] {
            assert_eq!(
                parse_command(bad),
                Err(ModError::BadCommand(bad.trim().to_string())),
                "input {bad:?} should be a BadCommand"
            );
        }
    }

    #[test]
    fn save_then_load_round_trips() {
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
    fn load_missing_file_gives_empty_store() {
        let path = temp_path("does_not_exist");
        let _ = std::fs::remove_file(&path);
        let list = ModList::load(&path).expect("a missing file is not an error");
        assert_eq!(list.mods().len(), 0);
    }

    #[test]
    fn run_executes_commands() {
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
