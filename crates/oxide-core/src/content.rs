//! The quest catalog: `content/tracks/<track>/worlds/<NN-slug>/<NN-slug>/quest.toml`.
//!
//! A quest's id is `<track>/<world>/<quest>`, e.g. `rust/01-foundations/03-functions`.
//! `requires` entries with two segments are relative to the quest's own track.
//! The legacy layout `content/worlds/...` (no `tracks/`) is read as the `rust` track.
//! A quest may carry flavours (`flavours/<name>/`): the same objectives told with a
//! different story, lesson, names and starter. `plain` is the quest folder itself.
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

/// How a quest's workspace is graded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Runner {
    /// `cargo test` in the workspace.
    #[default]
    Cargo,
    /// `tests.lua` in an embedded Lua 5.2, see `lua.rs`.
    Lua,
    /// `gradlew test` with JUnit XML, see `gradle.rs`.
    Gradle,
}

impl Runner {
    /// File extensions whose saves trigger a re-check.
    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            Runner::Cargo => &["rs", "toml"],
            Runner::Lua => &["lua", "json", "cfg"],
            Runner::Gradle => &["java", "kt", "kts", "gradle", "json", "toml", "properties"],
        }
    }

    /// Other files that count as the marker (a Gradle build may be Groovy).
    pub fn alt_marker(self) -> Option<&'static str> {
        match self {
            Runner::Gradle => Some("build.gradle"),
            _ => None,
        }
    }

    /// Whether `dir` holds this runner's project.
    pub fn is_project(self, dir: &Path) -> bool {
        dir.join(self.marker()).is_file() || self.alt_marker().is_some_and(|m| dir.join(m).is_file())
    }

    /// The file every starter/solution (and workspace) must have.
    pub fn marker(self) -> &'static str {
        match self {
            Runner::Cargo => "Cargo.toml",
            Runner::Lua => "tests.lua",
            Runner::Gradle => "build.gradle.kts",
        }
    }
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
    /// Omitted = the previous quest in the track (or the previous world's last quest).
    requires: Option<Vec<String>>,
    #[serde(default)]
    objectives: Vec<Objective>,
    #[serde(default)]
    hints: Vec<Hint>,
    #[serde(default)]
    quiz: Vec<Quiz>,
    /// The point of the quest is a compile error in the starter (borrow checker lessons).
    #[serde(default)]
    starter_fails_to_compile: bool,
    /// Defaults to the track's runner.
    runner: Option<Runner>,
    /// JDK major version for the gradle runner; overrides the track's.
    jdk: Option<u32>,
}

/// Overrides a flavour may give in its `quest.toml`; tests must stay the same.
#[derive(Debug, Deserialize, Default)]
struct VariantToml {
    title: Option<String>,
    summary: Option<String>,
    #[serde(default)]
    objectives: Vec<Objective>,
    #[serde(default)]
    hints: Vec<Hint>,
    #[serde(default)]
    quiz: Vec<Quiz>,
}

/// One flavour of a quest: `<quest>/flavours/<name>/` with its own lesson and files.
#[derive(Debug, Clone, Serialize)]
pub struct Variant {
    pub flavour: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub objectives: Option<Vec<Objective>>,
    pub hints: Option<Vec<Hint>>,
    pub quiz: Option<Vec<Quiz>>,
    pub dir: PathBuf,
}

pub const PLAIN: &str = "plain";

#[derive(Debug, Clone, Serialize)]
pub struct Quest {
    pub id: String,
    /// Which flavour this value tells: `plain`, or a name from `flavours/`.
    pub flavour: String,
    /// Other flavours available for this quest (empty once resolved).
    pub variants: Vec<Variant>,
    pub track_id: String,
    pub world_id: String,
    pub index: u32,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub kind: QuestKind,
    pub runner: Runner,
    /// JDK major version (gradle runner); quest overrides track, `None` = the engine default.
    pub jdk: Option<u32>,
    pub xp: u32,
    pub minutes: u32,
    pub tags: Vec<String>,
    pub requires: Vec<String>,
    pub objectives: Vec<Objective>,
    pub hints: Vec<Hint>,
    pub quiz: Vec<Quiz>,
    pub starter_fails_to_compile: bool,
    pub dir: PathBuf,
    /// The track's `lib/` folder (Lua mocks and helpers), if it has one.
    pub lib_dir: Option<PathBuf>,
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
    /// The id without the track: `01-foundations/03-functions`.
    pub fn local_id(&self) -> &str {
        self.id.split_once('/').map(|(_, rest)| rest).unwrap_or(&self.id)
    }

    /// Flavour names this quest offers besides `plain`.
    pub fn flavours(&self) -> impl Iterator<Item = &str> {
        self.variants.iter().map(|v| v.flavour.as_str())
    }

    /// This quest told in `flavour`; the plain quest when it has no such flavour.
    /// The result has `variants` emptied and `dir` pointing at the flavour's files.
    pub fn resolved(&self, flavour: &str) -> Quest {
        let mut q = self.clone();
        q.variants = Vec::new();
        let Some(v) = self.variants.iter().find(|v| v.flavour == flavour) else {
            return q;
        };
        q.flavour = v.flavour.clone();
        q.dir = v.dir.clone();
        if let Some(t) = &v.title {
            q.title = t.clone();
        }
        if let Some(s) = &v.summary {
            q.summary = s.clone();
        }
        if let Some(o) = &v.objectives {
            q.objectives = o.clone();
        }
        if let Some(h) = &v.hints {
            q.hints = h.clone();
        }
        if let Some(z) = &v.quiz {
            q.quiz = z.clone();
        }
        q
    }
}

#[derive(Debug, Deserialize)]
struct TrackToml {
    title: String,
    #[serde(default)]
    blurb: String,
    #[serde(default)]
    runner: Runner,
    /// JDK major version for gradle quests: 21 or 25.
    jdk: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub blurb: String,
    pub runner: Runner,
    pub jdk: Option<u32>,
    pub dir: PathBuf,
    pub lib_dir: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct WorldToml {
    title: String,
    #[serde(default)]
    blurb: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct World {
    /// `<track>/<NN-slug>`
    pub id: String,
    pub track_id: String,
    pub index: u32,
    pub slug: String,
    pub title: String,
    pub blurb: String,
    pub quests: Vec<Quest>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Catalog {
    pub tracks: Vec<Track>,
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

    pub fn track(&self, id: &str) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == id)
    }

    pub fn worlds_of<'a>(&'a self, track_id: &'a str) -> impl Iterator<Item = &'a World> + 'a {
        self.worlds.iter().filter(move |w| w.track_id == track_id)
    }

    /// Quests the learner can start now: not done, every requirement done.
    /// In catalog order, so `[0]` is "the next quest".
    pub fn available<'a>(&'a self, completed: &HashSet<String>) -> Vec<&'a Quest> {
        self.quests()
            .filter(|q| !completed.contains(&q.id))
            .filter(|q| q.requires.iter().all(|r| completed.contains(r)))
            .collect()
    }

    /// `available` limited to one track.
    pub fn available_in<'a>(&'a self, track_id: &str, completed: &HashSet<String>) -> Vec<&'a Quest> {
        self.available(completed).into_iter().filter(|q| q.track_id == track_id).collect()
    }

    pub fn total_xp(&self) -> u32 {
        self.quests().map(|q| q.xp).sum()
    }

    /// Every flavour name used anywhere, `plain` first.
    pub fn flavours(&self) -> Vec<String> {
        let mut names: Vec<String> = self.quests().flat_map(|q| q.flavours()).map(str::to_string).collect();
        names.sort();
        names.dedup();
        names.insert(0, PLAIN.to_string());
        names
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

/// Load and validate the catalog under `content_dir` (the folder holding `tracks/` or `worlds/`).
pub fn load(content_dir: &Path) -> Result<Catalog, Problems> {
    let mut problems = Vec::new();
    let mut tracks = Vec::new();
    let mut worlds = Vec::new();

    // (track, the folder holding its worlds)
    let mut specs: Vec<(Track, PathBuf)> = Vec::new();
    let tracks_dir = content_dir.join("tracks");
    if tracks_dir.is_dir() {
        for tdir in sorted_dirs(&tracks_dir, &mut problems) {
            let id = tdir.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            if id.is_empty() || !id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
                problems.push(format!("{}: track folder must be lowercase a-z0-9-", tdir.display()));
                continue;
            }
            let meta: TrackToml = match read_toml(&tdir.join("track.toml")) {
                Ok(m) => m,
                Err(e) => {
                    problems.push(e);
                    continue;
                }
            };
            let lib_dir = Some(tdir.join("lib")).filter(|p| p.is_dir());
            specs.push((
                Track { id, title: meta.title, blurb: meta.blurb, runner: meta.runner, jdk: meta.jdk, dir: tdir.clone(), lib_dir },
                tdir.join("worlds"),
            ));
        }
    }
    let legacy = content_dir.join("worlds");
    if legacy.is_dir() && !specs.iter().any(|(t, _)| t.id == "rust") {
        specs.push((
            Track {
                id: "rust".into(),
                title: "Rust".into(),
                blurb: "From hello world to shipping real apps.".into(),
                runner: Runner::Cargo,
                jdk: None,
                dir: content_dir.to_path_buf(),
                lib_dir: None,
            },
            legacy,
        ));
    }
    if specs.is_empty() {
        problems.push(format!("{}: no tracks/ or worlds/ folder", content_dir.display()));
    }

    for (track, worlds_dir) in specs {
        let mut track_worlds = Vec::new();
        for wdir in sorted_dirs(&worlds_dir, &mut problems) {
            let Some((index, slug)) = split_index(&wdir) else {
                problems.push(format!("{}: folder must be named NN-slug", wdir.display()));
                continue;
            };
            let world_id = format!("{}/{index:02}-{slug}", track.id);
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
                let (q, errs) = load_quest(&qdir, &track, &world_id, qindex, &qslug);
                problems.extend(errs);
                if let Some(q) = q {
                    quests.push(q);
                }
            }
            track_worlds.push(World {
                id: world_id,
                track_id: track.id.clone(),
                index,
                slug,
                title: meta.title,
                blurb: meta.blurb,
                quests,
            });
        }
        fill_implicit_requires(&mut track_worlds);
        worlds.extend(track_worlds);
        tracks.push(track);
    }

    validate_graph(&worlds, &mut problems);

    if problems.is_empty() {
        Ok(Catalog { tracks, worlds })
    } else {
        Err(Problems(problems))
    }
}

/// The quest (if quest.toml parsed at all) plus every problem found in it.
fn load_quest(dir: &Path, track: &Track, world_id: &str, index: u32, slug: &str) -> (Option<Quest>, Vec<String>) {
    let id = format!("{world_id}/{index:02}-{slug}");
    let toml_path = dir.join("quest.toml");
    let t: QuestToml = match read_toml(&toml_path) {
        Ok(t) => t,
        Err(e) => return (None, vec![e]),
    };
    let mut errs = Vec::new();
    let at = |msg: &str| format!("{}: {msg}", toml_path.display());
    let runner = t.runner.unwrap_or(track.runner);

    if t.title.trim().is_empty() {
        errs.push(at("title is empty"));
    }
    if t.jdk.is_some_and(|j| !matches!(j, 21 | 25)) {
        errs.push(at("jdk must be 21 or 25"));
    }
    if track.jdk.is_some_and(|j| !matches!(j, 21 | 25)) {
        errs.push(format!("{}: jdk must be 21 or 25", track.dir.join("track.toml").display()));
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
                if !runner.is_project(&dir.join(sub)) {
                    errs.push(format!("{}: {sub}/{} missing", dir.display(), runner.marker()));
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
    let variants = load_variants(dir, &t, runner, &mut errs);
    // `requires` omitted (not `[]`) = chain to the previous quest.
    // Two-segment ids are relative to this track.
    let requires = match t.requires {
        Some(r) => r
            .into_iter()
            .map(|r| if r.matches('/').count() == 1 { format!("{}/{r}", track.id) } else { r })
            .collect(),
        None => vec![IMPLICIT.to_string()],
    };
    let quest = Quest {
        id,
        flavour: PLAIN.to_string(),
        variants,
        track_id: track.id.clone(),
        world_id: world_id.to_string(),
        index,
        slug: slug.to_string(),
        title: t.title,
        summary: t.summary,
        kind: t.kind,
        runner,
        jdk: t.jdk.or(track.jdk),
        xp: t.xp,
        minutes: t.minutes,
        tags: t.tags,
        requires,
        objectives: t.objectives,
        hints: t.hints,
        quiz: t.quiz,
        starter_fails_to_compile: t.starter_fails_to_compile,
        dir: dir.to_path_buf(),
        lib_dir: track.lib_dir.clone(),
    };
    (Some(quest), errs)
}

/// `flavours/<name>/`: lesson.md required; starter/solution required for project quests;
/// objective tests must match the plain quest in order (labels may differ).
fn load_variants(dir: &Path, plain: &QuestToml, runner: Runner, errs: &mut Vec<String>) -> Vec<Variant> {
    let fdir = dir.join("flavours");
    if !fdir.is_dir() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for vdir in sorted_dirs(&fdir, errs) {
        let name = vdir.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        if name == PLAIN || !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            errs.push(format!("{}: flavour folder must be lowercase a-z0-9- and not `plain`", vdir.display()));
            continue;
        }
        let at = |msg: &str| format!("{}: {msg}", vdir.display());
        let toml_path = vdir.join("quest.toml");
        let vt: VariantToml = if toml_path.is_file() {
            match read_toml(&toml_path) {
                Ok(v) => v,
                Err(e) => {
                    errs.push(e);
                    continue;
                }
            }
        } else {
            VariantToml::default()
        };
        if !vdir.join("lesson.md").is_file() {
            errs.push(at("lesson.md missing"));
        }
        let is_project = matches!(plain.kind, QuestKind::Exercise | QuestKind::Boss);
        if is_project {
            for sub in ["starter", "solution"] {
                if !runner.is_project(&vdir.join(sub)) {
                    errs.push(at(&format!("{sub}/{} missing (a flavour of a project quest ships its own code)", runner.marker())));
                }
            }
        }
        if !vt.objectives.is_empty() {
            let plain_tests: Vec<&str> = plain.objectives.iter().map(|o| o.test.as_str()).collect();
            let mine: Vec<&str> = vt.objectives.iter().map(|o| o.test.as_str()).collect();
            if plain_tests != mine {
                errs.push(at(&format!(
                    "objective tests must match the plain quest in order: plain {plain_tests:?}, flavour {mine:?}"
                )));
            }
        }
        if !vt.quiz.is_empty() && vt.quiz.len() != plain.quiz.len() {
            errs.push(at("quiz must have as many questions as the plain quest"));
        }
        for (i, q) in vt.quiz.iter().enumerate() {
            if q.choices.len() < 2 || q.answer >= q.choices.len() {
                errs.push(at(&format!("quiz[{i}]: need 2+ choices and answer in range")));
            }
        }
        out.push(Variant {
            flavour: name,
            title: vt.title,
            summary: vt.summary,
            objectives: (!vt.objectives.is_empty()).then_some(vt.objectives),
            hints: (!vt.hints.is_empty()).then_some(vt.hints),
            quiz: (!vt.quiz.is_empty()).then_some(vt.quiz),
            dir: vdir,
        });
    }
    out
}

const IMPLICIT: &str = "<previous>";

/// `requires` omitted → previous quest in the track; first quest → last quest of the previous world.
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
                if p.is_dir() && !name.starts_with('.') && !matches!(name, "starter" | "solution" | "lib" | "worlds" | "flavours") {
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

    fn quest(worlds_root: &Path, world: &str, q: &str, extra: &str, marker: &str) {
        let d = worlds_root.join(world).join(q);
        write(
            &d.join("quest.toml"),
            &format!(
                "title = \"T\"\nkind = \"exercise\"\nxp = 10\nminutes = 5\n{extra}\n[[objectives]]\ntest = \"t\"\nlabel = \"L\"\n"
            ),
        );
        write(&d.join("lesson.md"), "# hi");
        write(&d.join("starter").join(marker), "x");
        write(&d.join("solution").join(marker), "x");
    }

    #[test]
    fn legacy_layout_is_the_rust_track() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let w = root.join("worlds");
        write(&w.join("00-tutorial/world.toml"), "title = \"Tutorial\"");
        write(&w.join("01-basics/world.toml"), "title = \"Basics\"");
        quest(&w, "00-tutorial", "01-hello", "", "Cargo.toml");
        quest(&w, "00-tutorial", "02-loop", "", "Cargo.toml");
        quest(&w, "01-basics", "01-vars", "", "Cargo.toml");
        quest(&w, "01-basics", "02-free", "requires = []", "Cargo.toml");
        quest(&w, "01-basics", "03-rel", "requires = [\"00-tutorial/01-hello\"]", "Cargo.toml");

        let cat = load(root).unwrap();
        assert_eq!(cat.tracks.len(), 1);
        assert_eq!(cat.tracks[0].id, "rust");
        let ids: Vec<_> = cat.quests().map(|q| q.id.clone()).collect();
        assert_eq!(
            ids,
            [
                "rust/00-tutorial/01-hello",
                "rust/00-tutorial/02-loop",
                "rust/01-basics/01-vars",
                "rust/01-basics/02-free",
                "rust/01-basics/03-rel"
            ]
        );
        assert_eq!(cat.quest("rust/00-tutorial/01-hello").unwrap().requires, Vec::<String>::new());
        assert_eq!(cat.quest("rust/00-tutorial/02-loop").unwrap().requires, ["rust/00-tutorial/01-hello"]);
        assert_eq!(cat.quest("rust/01-basics/01-vars").unwrap().requires, ["rust/00-tutorial/02-loop"]);
        assert!(cat.quest("rust/01-basics/02-free").unwrap().requires.is_empty());
        assert_eq!(cat.quest("rust/01-basics/03-rel").unwrap().requires, ["rust/00-tutorial/01-hello"]);
        assert_eq!(cat.quest("rust/01-basics/03-rel").unwrap().local_id(), "01-basics/03-rel");

        let done = HashSet::new();
        let avail: Vec<_> = cat.available(&done).iter().map(|q| q.id.as_str()).collect();
        assert_eq!(avail, ["rust/00-tutorial/01-hello", "rust/01-basics/02-free"]);
    }

    #[test]
    fn tracks_layout_with_runner_and_cross_track_requires() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(&root.join("tracks/lua/track.toml"), "title = \"Lua\"\nrunner = \"lua\"");
        write(&root.join("tracks/lua/lib/mock.lua"), "return {}");
        write(&root.join("tracks/rust/track.toml"), "title = \"Rust\"");
        let lw = root.join("tracks/lua/worlds");
        let rw = root.join("tracks/rust/worlds");
        write(&lw.join("01-core/world.toml"), "title = \"Core\"");
        write(&rw.join("00-tutorial/world.toml"), "title = \"Tutorial\"");
        quest(&rw, "00-tutorial", "01-hello", "", "Cargo.toml");
        quest(&lw, "01-core", "01-values", "requires = [\"rust/00-tutorial/01-hello\"]", "tests.lua");
        quest(&lw, "01-core", "02-tables", "", "tests.lua");

        let cat = load(root).unwrap();
        assert_eq!(cat.tracks.iter().map(|t| t.id.as_str()).collect::<Vec<_>>(), ["lua", "rust"]);
        let q = cat.quest("lua/01-core/01-values").unwrap();
        assert_eq!(q.runner, Runner::Lua);
        assert!(q.lib_dir.is_some());
        assert_eq!(q.requires, ["rust/00-tutorial/01-hello"]);
        assert_eq!(cat.quest("lua/01-core/02-tables").unwrap().requires, ["lua/01-core/01-values"]);
        let done: HashSet<String> = ["rust/00-tutorial/01-hello".to_string()].into();
        let avail: Vec<_> = cat.available_in("lua", &done).iter().map(|q| q.id.as_str()).collect();
        assert_eq!(avail, ["lua/01-core/01-values"]);
    }

    #[test]
    fn flavours_overlay_the_plain_quest() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let w = root.join("worlds");
        write(&w.join("00-tutorial/world.toml"), "title = \"Tutorial\"");
        quest(&w, "00-tutorial", "01-hello", "", "Cargo.toml");
        let f = w.join("00-tutorial/01-hello/flavours/games");
        write(&f.join("quest.toml"), "title = \"Name tag\"\n[[objectives]]\ntest = \"t\"\nlabel = \"tag shows the name\"\n");
        write(&f.join("lesson.md"), "# gamey");
        write(&f.join("starter/Cargo.toml"), "x");
        write(&f.join("solution/Cargo.toml"), "x");
        let cat = load(root).unwrap();
        assert_eq!(cat.flavours(), ["plain", "games"]);
        let q = cat.quest("rust/00-tutorial/01-hello").unwrap();
        assert_eq!(q.flavour, "plain");
        assert_eq!(q.title, "T");
        let g = q.resolved("games");
        assert_eq!(g.flavour, "games");
        assert_eq!(g.title, "Name tag");
        assert_eq!(g.objectives[0].label, "tag shows the name");
        assert!(g.starter_dir().ends_with("flavours/games/starter"));
        assert!(g.variants.is_empty());
        let p = q.resolved("nope");
        assert_eq!(p.flavour, "plain");
        assert_eq!(p.title, "T");

        // A flavour with a different test name is rejected.
        write(&f.join("quest.toml"), "[[objectives]]\ntest = \"other\"\nlabel = \"x\"\n");
        let err = load(root).unwrap_err().to_string();
        assert!(err.contains("objective tests must match"), "{err}");
    }

    #[test]
    fn reports_bad_requires_and_missing_files() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let w = root.join("worlds");
        write(&w.join("00-tutorial/world.toml"), "title = \"Tutorial\"");
        quest(&w, "00-tutorial", "01-hello", "requires = [\"nope/01-x\"]", "Cargo.toml");
        fs::remove_file(w.join("00-tutorial/01-hello/lesson.md")).unwrap();
        let err = load(root).unwrap_err();
        let text = err.to_string();
        assert!(text.contains("lesson.md missing"), "{text}");
        assert!(text.contains("requires unknown quest `rust/nope/01-x`"), "{text}");
    }
}
