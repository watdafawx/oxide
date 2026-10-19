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
        // TODO: an empty ModList (hint: it derives Default)
        todo!()
    }

    pub fn mods(&self) -> &[ModEntry] {
        &self.mods
    }

    /// Add a mod (enabled) and return its id. Ids start at 1 and go up (highest existing id + 1).
    pub fn add(&mut self, name: &str) -> u32 {
        // TODO: next id = highest id + 1 (1 when empty); push a ModEntry with enabled: true
        todo!()
    }

    /// Flip the `enabled` flag of mod `id`. Unknown id -> ModError::NotFound(id).
    pub fn toggle(&mut self, id: u32) -> Result<(), ModError> {
        // TODO: find the mod with this id (iter_mut().find), flip `enabled`, or return NotFound
        todo!()
    }

    /// Read a mod list from `path`. A file that does not exist yet is an empty list.
    pub fn load(path: &Path) -> anyhow::Result<ModList> {
        // TODO: std::fs::read_to_string(path). If the error kind is std::io::ErrorKind::NotFound return Ok(ModList::new()); other errors get .with_context(). Then parse each line with parse_line(line) and add context saying which line number failed.
        todo!()
    }

    /// Write the mod list to `path`, one `format_line` per mod.
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        // TODO: join format_line for each mod with '\n' and std::fs::write it, with .with_context(|| format!("writing {}", path.display()))
        todo!()
    }
}

/// Turn "add sodium" / "toggle 3" / "list" into a Command.
pub fn parse_command(input: &str) -> Result<Command, ModError> {
    // TODO: trim; split_once(' ') into verb + rest; match the verb. "add" needs a non-empty name, "toggle" needs a number, anything else -> BadCommand(trimmed input)
    todo!()
}

/// Run a command. Add -> "added <id>", Toggle -> "toggled <id>",
/// List -> one line per mod like "[x] 1 sodium" (enabled) / "[ ] 2 lithium" (disabled), or "no mods yet".
pub fn run(list: &mut ModList, cmd: Command) -> anyhow::Result<String> {
    // TODO: match cmd. Use `list.toggle(id)?` so ModError turns into anyhow::Error.
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
