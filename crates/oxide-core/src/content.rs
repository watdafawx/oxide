//! The quest catalog: `content/worlds/<NN-slug>/<NN-slug>/quest.toml`.
//!
//! A quest's id is its path under `worlds/`, e.g. `01-foundations/03-functions`.
//! See `docs/CONTENT.md` for the authoring rules this module enforces.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuestKind {
    /// Starter project with failing tests; done when every objective passes.
    Exercise,
    /// Same as exercise, bigger, ends a world.
    Boss,
    /// Lesson only; done when the quiz is answered.
    Read,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    /// Test function name (suffix match: `borrows` matches `tests::borrows`).
    pub test: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub question: String,
    pub choices: Vec<String>,
    /// Index into `choices`.
    pub answer: usize,
}

#[derive(Debug, Deserialize)]
struct QuestToml {
    title: String,
    #[serde(default)]
    summary: String,
    kind: QuestKind,
    xp: u32,
    minutes: u32,
    #[serde(default)]
    tags: Vec<String>,
    /// Omitted = the previous quest in the world (or the previous world's last quest).
    requires: Option<Vec<String>>,
    #[serde(default)]
    objectives: Vec<Objective>,
    #[serde(default)]
    hints: Vec<Hint>,
    #[serde(default)]
    quiz: Vec<Quiz>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Quest {
    pub id: String,
    pub world_id: String,
    pub index: u32,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub kind: QuestKind,
    pub xp: u32,
    pub minutes: u32,
    pub tags: Vec<String>,
    pub requires: Vec<String>,
    pub objectives: Vec<Objective>,
    pub hints: Vec<Hint>,
    pub quiz: Vec<Quiz>,
    pub dir: PathBuf,
}

impl Quest {
    pub fn lesson(&self) -> std::io::Result<String> {
        fs::read_to_string(self.dir.join("lesson.md"))
    }
    pub fn starter_dir(&self) -> PathBuf {
        self.dir.join("starter")
    }
    pub fn solution_dir(&self) -> PathBuf {
        self.dir.join("solution")
    }
    pub fn has_project(&self) -> bool {
        matches!(self.kind, QuestKind::Exercise | QuestKind::Boss)
    }
}

#[derive(Debug, Deserialize)]
struct WorldToml {
    title: String,
    #[serde(default)]
    blurb: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct World {
    pub id: String,
    pub index: u32,
    pub slug: String,
    pub title: String,
    pub blurb: String,
    pub quests: Vec<Quest>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Catalog {
    pub worlds: Vec<World>,
}

impl Catalog {
    pub fn quests(&self) -> impl Iterator<Item = &Quest> {
        self.worlds.iter().flat_map(|w| w.quests.iter())
    }

    pub fn quest(&self, id: &str) -> Option<&Quest> {
        self.quests().find(|q| q.id == id)
    }

    pub fn world(&self, id: &str) -> Option<&World> {
        self.worlds.iter().find(|w| w.id == id)
    }

    /// Quests the learner can start now: not done, every requirement done.
    /// In catalog order, so `[0]` is "the next quest".
    pub fn available<'a>(&'a self, completed: &HashSet<String>) -> Vec<&'a Quest> {
        self.quests()
            .filter(|q| !completed.contains(&q.id))
            .filter(|q| q.requires.iter().all(|r| completed.contains(r)))
            .collect()
    }

    pub fn total_xp(&self) -> u32 {
        self.quests().map(|q| q.xp).sum()
    }
}

/// Every problem found while loading, with the file it belongs to.
#[derive(Debug, Default)]
pub struct Problems(pub Vec<String>);

impl fmt::Display for Problems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in &self.0 {
            writeln!(f, "{p}")?;
        }
        Ok(())
    }
}
impl std::error::Error for Problems {}

/// Load and validate the catalog under `content_dir` (the folder holding `worlds/`).
pub fn load(content_dir: &Path) -> Result<Catalog, Problems> {
    let mut problems = Vec::new();
    let worlds_dir = content_dir.join("worlds");
    let mut worlds = Vec::new();

    for wdir in sorted_dirs(&worlds_dir, &mut problems) {
        let Some((index, slug)) = split_index(&wdir) else {
            problems.push(format!("{}: folder must be named NN-slug", wdir.display()));
            continue;
        };
        let world_id = format!("{index:02}-{slug}");
        let meta: WorldToml = match read_toml(&wdir.join("world.toml")) {
            Ok(m) => m,
            Err(e) => {
                problems.push(e);
                continue;
            }
        };
        let mut quests = Vec::new();
        for qdir in sorted_dirs(&wdir, &mut problems) {
            let Some((qindex, qslug)) = split_index(&qdir) else {
                problems.push(format!("{}: folder must be named NN-slug", qdir.display()));
                continue;
            };
            let (q, errs) = load_quest(&qdir, &world_id, qindex, &qslug);
            problems.extend(errs);
            if let Some(q) = q {
                quests.push(q);
            }
        }
        worlds.push(World { id: world_id, index, slug, title: meta.title, blurb: meta.blurb, quests });
    }

    fill_implicit_requires(&mut worlds);
    validate_graph(&worlds, &mut problems);

    if problems.is_empty() {
        Ok(Catalog { worlds })
    } else {
        Err(Problems(problems))
    }
}

/// The quest (if quest.toml parsed at all) plus every problem found in it.
fn load_quest(dir: &Path, world_id: &str, index: u32, slug: &str) -> (Option<Quest>, Vec<String>) {
    let id = format!("{world_id}/{index:02}-{slug}");
    let toml_path = dir.join("quest.toml");
    let t: QuestToml = match read_toml(&toml_path) {
        Ok(t) => t,
        Err(e) => return (None, vec![e]),
    };
    let mut errs = Vec::new();
    let at = |msg: &str| format!("{}: {msg}", toml_path.display());

    if t.title.trim().is_empty() {
        errs.push(at("title is empty"));
    }
    if t.minutes == 0 {
        errs.push(at("minutes must be > 0"));
    }
    if !dir.join("lesson.md").is_file() {
        errs.push(format!("{}: lesson.md missing", dir.display()));
    }
    match t.kind {
        QuestKind::Exercise | QuestKind::Boss => {
            if t.objectives.is_empty() {
                errs.push(at("exercise/boss needs at least one [[objectives]]"));
            }
            for sub in ["starter", "solution"] {
                if !dir.join(sub).join("Cargo.toml").is_file() {
                    errs.push(format!("{}: {sub}/Cargo.toml missing", dir.display()));
                }
            }
            let mut seen = HashSet::new();
            for o in &t.objectives {
                if !seen.insert(&o.test) {
                    errs.push(at(&format!("duplicate objective test `{}`", o.test)));
                }
            }
        }
        QuestKind::Read => {
            if t.quiz.is_empty() {
                errs.push(at("read quest needs at least one [[quiz]]"));
            }
            if !t.objectives.is_empty() {
                errs.push(at("read quest cannot have objectives"));
            }
        }
    }
    for (i, q) in t.quiz.iter().enumerate() {
        if q.choices.len() < 2 || q.answer >= q.choices.len() {
            errs.push(at(&format!("quiz[{i}]: need 2+ choices and answer in range")));
        }
    }
    // `requires` omitted (not `[]`) = chain to the previous quest.
    let requires = match t.requires {
        Some(r) => r,
        None => vec![IMPLICIT.to_string()],
    };
    let quest = Quest {
        id,
        world_id: world_id.to_string(),
        index,
        slug: slug.to_string(),
        title: t.title,
        summary: t.summary,
        kind: t.kind,
        xp: t.xp,
        minutes: t.minutes,
        tags: t.tags,
        requires,
        objectives: t.objectives,
        hints: t.hints,
        quiz: t.quiz,
        dir: dir.to_path_buf(),
    };
    (Some(quest), errs)
}

const IMPLICIT: &str = "<previous>";

/// `requires` omitted → previous quest in the world; first quest → last quest of the previous world.
fn fill_implicit_requires(worlds: &mut [World]) {
    let mut prev: Option<String> = None;
    for w in worlds.iter_mut() {
        for q in w.quests.iter_mut() {
            if q.requires.iter().any(|r| r == IMPLICIT) {
                q.requires.retain(|r| r != IMPLICIT);
                if let Some(p) = &prev {
                    q.requires.push(p.clone());
                }
            }
            prev = Some(q.id.clone());
        }
    }
}

fn validate_graph(worlds: &[World], problems: &mut Vec<String>) {
    let ids: HashMap<&str, &Quest> = worlds.iter().flat_map(|w| &w.quests).map(|q| (q.id.as_str(), q)).collect();
    for q in ids.values() {
        for r in &q.requires {
            if !ids.contains_key(r.as_str()) {
                problems.push(format!("{}: requires unknown quest `{r}`", q.id));
            }
        }
    }
    // Cycle check: DFS with colours.
    fn visit<'a>(
        id: &'a str,
        ids: &HashMap<&'a str, &'a Quest>,
        state: &mut HashMap<&'a str, u8>,
        problems: &mut Vec<String>,
    ) {
        match state.get(id) {
            Some(2) => return,
            Some(1) => {
                problems.push(format!("{id}: requirement cycle"));
                return;
            }
            _ => {}
        }
        state.insert(id, 1);
        if let Some(q) = ids.get(id) {
            for r in &q.requires {
                if ids.contains_key(r.as_str()) {
                    visit(r, ids, state, problems);
                }
            }
        }
        state.insert(id, 2);
    }
    let mut state = HashMap::new();
    for id in ids.keys() {
        visit(id, &ids, &mut state, problems);
    }
}

fn read_toml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn sorted_dirs(dir: &Path, problems: &mut Vec<String>) -> Vec<PathBuf> {
    let mut out = Vec::new();
    match fs::read_dir(dir) {
        Ok(rd) => {
            for e in rd.flatten() {
                let p = e.path();
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if p.is_dir() && !name.starts_with('.') && name != "starter" && name != "solution" {
                    out.push(p);
                }
            }
        }
        Err(e) => problems.push(format!("{}: {e}", dir.display())),
    }
    out.sort();
    out
}

/// `03-functions` → `(3, "functions")`.
fn split_index(dir: &Path) -> Option<(u32, String)> {
    let name = dir.file_name()?.to_str()?;
    let (num, slug) = name.split_once('-')?;
    let index = num.parse().ok()?;
    if slug.is_empty() || !slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return None;
    }
    Some((index, slug.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(p: &Path, s: &str) {
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, s).unwrap();
    }

    fn quest(root: &Path, world: &str, q: &str, extra: &str) {
        let d = root.join("worlds").join(world).join(q);
        write(
            &d.join("quest.toml"),
            &format!(
                "title = \"T\"\nkind = \"exercise\"\nxp = 10\nminutes = 5\n{extra}\n[[objectives]]\ntest = \"t\"\nlabel = \"L\"\n"
            ),
        );
        write(&d.join("lesson.md"), "# hi");
        write(&d.join("starter/Cargo.toml"), "[package]");
        write(&d.join("solution/Cargo.toml"), "[package]");
    }

    #[test]
    fn implicit_chain_and_available() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(&root.join("worlds/00-tutorial/world.toml"), "title = \"Tutorial\"");
        write(&root.join("worlds/01-basics/world.toml"), "title = \"Basics\"");
        quest(root, "00-tutorial", "01-hello", "");
        quest(root, "00-tutorial", "02-loop", "");
        quest(root, "01-basics", "01-vars", "");
        quest(root, "01-basics", "02-free", "requires = []");

        let cat = load(root).unwrap();
        let ids: Vec<_> = cat.quests().map(|q| q.id.clone()).collect();
        assert_eq!(ids, ["00-tutorial/01-hello", "00-tutorial/02-loop", "01-basics/01-vars", "01-basics/02-free"]);
        assert_eq!(cat.quest("00-tutorial/01-hello").unwrap().requires, Vec::<String>::new());
        assert_eq!(cat.quest("00-tutorial/02-loop").unwrap().requires, ["00-tutorial/01-hello"]);
        assert_eq!(cat.quest("01-basics/01-vars").unwrap().requires, ["00-tutorial/02-loop"]);
        assert!(cat.quest("01-basics/02-free").unwrap().requires.is_empty());

        let done = HashSet::new();
        let avail: Vec<_> = cat.available(&done).iter().map(|q| q.id.as_str()).collect();
        assert_eq!(avail, ["00-tutorial/01-hello", "01-basics/02-free"]);
    }

    #[test]
    fn reports_bad_requires_and_missing_files() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(&root.join("worlds/00-tutorial/world.toml"), "title = \"Tutorial\"");
        quest(root, "00-tutorial", "01-hello", "requires = [\"nope/01-x\"]");
        fs::remove_file(root.join("worlds/00-tutorial/01-hello/lesson.md")).unwrap();
        let err = load(root).unwrap_err();
        let text = err.to_string();
        assert!(text.contains("lesson.md missing"), "{text}");
        assert!(text.contains("requires unknown quest"), "{text}");
    }
}
