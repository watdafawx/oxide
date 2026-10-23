//! Named profiles: each has its own progress database (XP, quests, achievements, settings) and its own
//! quests folder (the learner's code). The list lives in `<data>/profiles.json`; a new profile's files
//! go in `<data>/profiles/<id>/`. Profiles move between PCs as one zip ([`Profiles::export`] /
//! [`Profiles::import`]).

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::progress::Progress;

/// Folders a quest leaves behind that are build output, never the learner's work.
const SKIP: &[&str] = &["target", ".target", "build", ".gradle", "node_modules"];
/// Marks a zip as an Oxide profile, and says what is in it.
const MANIFEST: &str = "oxide-profile.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileEntry {
    pub id: String,
    pub name: String,
    /// Day it was made, `YYYY-MM-DD`.
    pub created: String,
    /// Set for the profile made from a save older than profiles: its files stay where they were.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quests: Option<PathBuf>,
}

/// What the app shows for a profile.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ProfileInfo {
    pub id: String,
    pub name: String,
    pub created: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Index {
    active: String,
    profiles: Vec<ProfileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    name: String,
    created: String,
    oxide: String,
}

pub struct Profiles {
    root: PathBuf,
    index: Index,
}

impl Profiles {
    /// Read `<root>/profiles.json`, or start it. On first start, an existing save (`legacy`: its progress
    /// database and quests folder) becomes the first profile, with its files left where they are.
    pub fn load(root: &Path, legacy: Option<(PathBuf, PathBuf)>) -> Result<Self> {
        let path = root.join("profiles.json");
        if let Ok(text) = fs::read_to_string(&path) {
            let index: Index = serde_json::from_str(&text).with_context(|| format!("reading {}", path.display()))?;
            if index.profiles.iter().any(|p| p.id == index.active) {
                return Ok(Profiles { root: root.to_path_buf(), index });
            }
        }
        let first = match legacy.filter(|(db, _)| db.is_file()) {
            Some((db, quests)) => ProfileEntry { id: "player".into(), name: "Player".into(), created: today(), db: Some(db), quests: Some(quests) },
            None => ProfileEntry { id: "player".into(), name: "Player".into(), created: today(), db: None, quests: None },
        };
        let profiles = Profiles { root: root.to_path_buf(), index: Index { active: first.id.clone(), profiles: vec![first] } };
        profiles.save()?;
        Ok(profiles)
    }

    fn save(&self) -> Result<()> {
        fs::create_dir_all(&self.root)?;
        let path = self.root.join("profiles.json");
        fs::write(&path, serde_json::to_string_pretty(&self.index)?).with_context(|| format!("writing {}", path.display()))
    }

    pub fn list(&self) -> Vec<ProfileInfo> {
        self.index
            .profiles
            .iter()
            .map(|p| ProfileInfo { id: p.id.clone(), name: p.name.clone(), created: p.created.clone(), active: p.id == self.index.active })
            .collect()
    }

    pub fn active(&self) -> &ProfileEntry {
        self.entry(&self.index.active).expect("load keeps the active profile in the list")
    }

    fn entry(&self, id: &str) -> Option<&ProfileEntry> {
        self.index.profiles.iter().find(|p| p.id == id)
    }

    /// A profile's own folder, for profiles made here.
    fn home(&self, id: &str) -> PathBuf {
        self.root.join("profiles").join(id)
    }

    /// Where a profile keeps its progress database.
    pub fn db_path(&self, id: &str) -> PathBuf {
        self.entry(id).and_then(|p| p.db.clone()).unwrap_or_else(|| self.home(id).join("progress.db"))
    }

    /// Where a profile keeps the learner's quest folders.
    pub fn quests_path(&self, id: &str) -> PathBuf {
        self.entry(id).and_then(|p| p.quests.clone()).unwrap_or_else(|| self.home(id).join("quests"))
    }

    /// Add a profile; gives back its id.
    pub fn create(&mut self, name: &str) -> Result<String> {
        let name = clean_name(name)?;
        let id = self.unique_id(&name);
        self.index.profiles.push(ProfileEntry { id: id.clone(), name, created: today(), db: None, quests: None });
        self.save()?;
        Ok(id)
    }

    pub fn rename(&mut self, id: &str, name: &str) -> Result<()> {
        let name = clean_name(name)?;
        let entry = self.index.profiles.iter_mut().find(|p| p.id == id).context("no such profile")?;
        entry.name = name;
        self.save()
    }

    pub fn set_active(&mut self, id: &str) -> Result<()> {
        if self.entry(id).is_none() {
            bail!("no such profile");
        }
        self.index.active = id.to_string();
        self.save()
    }

    /// Remove a profile and its files. Never the active one. A profile made from an older save keeps its
    /// files where they were; it only leaves the list.
    pub fn delete(&mut self, id: &str) -> Result<()> {
        if id == self.index.active {
            bail!("switch to another profile before deleting this one");
        }
        let pos = self.index.profiles.iter().position(|p| p.id == id).context("no such profile")?;
        let entry = self.index.profiles.remove(pos);
        self.save()?;
        if entry.db.is_none() && entry.quests.is_none() {
            let home = self.home(id);
            if home.is_dir() {
                fs::remove_dir_all(&home).with_context(|| format!("deleting {}", home.display()))?;
            }
        }
        Ok(())
    }

    /// Write a profile to one zip: its name, a snapshot of its progress, and its quest code (no build output).
    /// `open` is the profile's open database when it is the active one, so the snapshot is consistent.
    pub fn export(&self, id: &str, open: Option<&Progress>, dest: &Path) -> Result<()> {
        let entry = self.entry(id).context("no such profile")?;
        let snapshot = std::env::temp_dir().join(format!("oxide-export-{}-{}.db", id, std::process::id()));
        let _ = fs::remove_file(&snapshot);
        match open {
            Some(progress) => progress.backup_to(&snapshot)?,
            None => {
                let db = self.db_path(id);
                if db.is_file() {
                    Progress::open(&db)?.backup_to(&snapshot)?;
                } else {
                    Progress::open(&snapshot)?;
                }
            }
        }
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = fs::File::create(dest).with_context(|| format!("creating {}", dest.display()))?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        let manifest = Manifest { name: entry.name.clone(), created: entry.created.clone(), oxide: env!("CARGO_PKG_VERSION").into() };
        zip.start_file(MANIFEST, options)?;
        zip.write_all(serde_json::to_string_pretty(&manifest)?.as_bytes())?;
        zip.start_file("progress.db", options)?;
        zip.write_all(&fs::read(&snapshot)?)?;
        let _ = fs::remove_file(&snapshot);
        let quests = self.quests_path(id);
        if quests.is_dir() {
            add_dir(&mut zip, &quests, "quests", options)?;
        }
        zip.finish()?;
        Ok(())
    }

    /// Add the profile in an exported zip as a new profile (its name gets " (2)" and so on when taken).
    /// Gives back the new id. Nothing outside the new profile's folder is written.
    pub fn import(&mut self, zip_bytes: &[u8]) -> Result<String> {
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(zip_bytes)).context("that file is not a zip")?;
        let manifest: Manifest = {
            let mut text = String::new();
            zip.by_name(MANIFEST).context("that zip is not an Oxide profile (no oxide-profile.json)")?.read_to_string(&mut text)?;
            serde_json::from_str(&text).context("oxide-profile.json is broken")?
        };
        let name = self.free_name(&clean_name(&manifest.name).unwrap_or_else(|_| "Imported".into()));
        let id = self.unique_id(&name);
        let home = self.home(&id);
        fs::create_dir_all(&home)?;
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            // `enclosed_name` refuses absolute paths and `..`, so a crafted zip cannot write outside `home`.
            let Some(rel) = file.enclosed_name() else { continue };
            let keep = rel == Path::new("progress.db") || rel.starts_with("quests");
            if !keep || file.is_dir() {
                continue;
            }
            let out = home.join(&rel);
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            fs::write(&out, bytes)?;
        }
        if !home.join("progress.db").is_file() {
            let _ = fs::remove_dir_all(&home);
            bail!("that zip has no progress.db");
        }
        Progress::open(&home.join("progress.db")).context("the progress in that zip does not open")?;
        self.index.profiles.push(ProfileEntry { id: id.clone(), name, created: manifest.created, db: None, quests: None });
        self.save()?;
        Ok(id)
    }

    fn unique_id(&self, name: &str) -> String {
        let base: String = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");
        let base = if base.is_empty() { "profile".to_string() } else { base };
        let taken = |id: &str| self.entry(id).is_some() || self.home(id).exists();
        if !taken(&base) {
            return base;
        }
        (2..).map(|n| format!("{base}-{n}")).find(|id| !taken(id)).expect("some number is free")
    }

    fn free_name(&self, name: &str) -> String {
        let taken = |n: &str| self.index.profiles.iter().any(|p| p.name.eq_ignore_ascii_case(n));
        if !taken(name) {
            return name.to_string();
        }
        (2..).map(|n| format!("{name} ({n})")).find(|n| !taken(n)).expect("some number is free")
    }
}

fn clean_name(name: &str) -> Result<String> {
    let name = name.trim();
    if name.is_empty() {
        bail!("a profile needs a name");
    }
    if name.chars().count() > 40 {
        bail!("keep the name to 40 characters");
    }
    Ok(name.to_string())
}

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn add_dir(zip: &mut zip::ZipWriter<fs::File>, dir: &Path, prefix: &str, options: zip::write::SimpleFileOptions) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let path = entry.path();
        let inside = format!("{prefix}/{name}");
        if entry.file_type()?.is_dir() {
            if !SKIP.contains(&name.as_ref()) {
                add_dir(zip, &path, &inside, options)?;
            }
        } else {
            zip.start_file(&inside, options)?;
            zip.write_all(&fs::read(&path)?)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_start_adopts_an_older_save() {
        let tmp = tempfile::tempdir().unwrap();
        let old_db = tmp.path().join("old/progress.db");
        Progress::open(&old_db).unwrap();
        let old_quests = tmp.path().join("oxide-quests");
        let profiles = Profiles::load(&tmp.path().join("data"), Some((old_db.clone(), old_quests.clone()))).unwrap();
        assert_eq!(profiles.list().len(), 1);
        assert_eq!(profiles.db_path("player"), old_db);
        assert_eq!(profiles.quests_path("player"), old_quests);
        let again = Profiles::load(&tmp.path().join("data"), None).unwrap();
        assert_eq!(again.db_path("player"), old_db, "profiles.json remembers it");
    }

    #[test]
    fn create_switch_rename_delete() {
        let tmp = tempfile::tempdir().unwrap();
        let mut profiles = Profiles::load(tmp.path(), None).unwrap();
        let id = profiles.create("Sam Two").unwrap();
        assert_eq!(id, "sam-two");
        assert_eq!(profiles.create("Sam Two").unwrap(), "sam-two-2");
        assert_eq!(profiles.db_path(&id), tmp.path().join("profiles/sam-two/progress.db"));
        profiles.set_active(&id).unwrap();
        assert!(profiles.delete(&id).is_err(), "never the active one");
        profiles.rename(&id, "Sam").unwrap();
        assert_eq!(Profiles::load(tmp.path(), None).unwrap().active().name, "Sam");
        Progress::open(&profiles.db_path("sam-two-2")).unwrap();
        profiles.delete("sam-two-2").unwrap();
        assert!(!tmp.path().join("profiles/sam-two-2").exists());
        assert!(profiles.create("  ").is_err());
    }

    #[test]
    fn export_then_import_keeps_progress_and_code() {
        let tmp = tempfile::tempdir().unwrap();
        let mut profiles = Profiles::load(tmp.path(), None).unwrap();
        let db = profiles.db_path("player");
        let progress = Progress::open(&db).unwrap();
        progress.start_quest("rust/00-tutorial/01-hello", chrono::Local::now()).unwrap();
        let quest = profiles.quests_path("player").join("rust/00-tutorial/01-hello");
        fs::create_dir_all(quest.join("src")).unwrap();
        fs::create_dir_all(quest.join("target/debug")).unwrap();
        fs::write(quest.join("src/lib.rs"), "// my code").unwrap();
        fs::write(quest.join("target/debug/big.bin"), "build output").unwrap();

        let zip = tmp.path().join("out/player.oxide-profile.zip");
        profiles.export("player", Some(&progress), &zip).unwrap();
        let id = profiles.import(&fs::read(&zip).unwrap()).unwrap();
        assert_eq!(id, "player-2");
        assert_eq!(profiles.list()[1].name, "Player (2)");
        let copied = profiles.quests_path(&id).join("rust/00-tutorial/01-hello");
        assert_eq!(fs::read_to_string(copied.join("src/lib.rs")).unwrap(), "// my code");
        assert!(!copied.join("target").exists(), "build output is left out");
        let imported = Progress::open(&profiles.db_path(&id)).unwrap();
        assert!(imported.run("rust/00-tutorial/01-hello").unwrap().is_some());
    }

    #[test]
    fn import_refuses_other_zips() {
        let tmp = tempfile::tempdir().unwrap();
        let mut profiles = Profiles::load(tmp.path(), None).unwrap();
        assert!(profiles.import(b"not a zip").is_err());
        let mut buf = std::io::Cursor::new(Vec::new());
        let mut zip = zip::ZipWriter::new(&mut buf);
        zip.start_file("hello.txt", zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(b"hi").unwrap();
        zip.finish().unwrap();
        let err = profiles.import(buf.get_ref()).unwrap_err().to_string();
        assert!(err.contains("not an Oxide profile"), "{err}");
        assert_eq!(profiles.list().len(), 1);
    }
}
