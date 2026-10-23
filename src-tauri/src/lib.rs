//! Tauri commands and events over `oxide_core`. The contract is `docs/API.md`.

mod files;
mod mcp;
mod run;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::Local;
use oxide_core::content::{Catalog, Quest, QuestKind, Track};
use oxide_core::grader::{self, CheckResult};
use oxide_core::profiles::{ProfileInfo, Profiles};
use oxide_core::progress::{Completion, Profile, Progress, Settings, ACHIEVEMENTS};
use oxide_core::{watcher, workspace};
use serde::Serialize;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager, State};

type CmdResult<T> = Result<T, String>;

fn err<E: std::fmt::Display>(e: E) -> String {
    format!("{e:#}")
}

struct FocusTimer {
    id: i64,
    quest_id: Option<String>,
    cancel: Arc<AtomicBool>,
}

struct AppState {
    catalog: Catalog,
    progress: Mutex<Progress>,
    /// The active profile's quests folder; swapped by `switch_profile`.
    quests_root: Mutex<PathBuf>,
    profiles: Mutex<Profiles>,
    /// Where exports go: next to oxide.exe for the portable zip, Documents otherwise.
    exports_dir: PathBuf,
    /// Shared by every profile, so switching never recompiles dependencies.
    target_dir: PathBuf,
    /// The one workspace being watched (the quest the learner has open).
    watch: Mutex<Option<(String, watcher::Handle)>>,
    last_checks: Mutex<HashMap<String, CheckResult>>,
    /// Quests with a cargo run in flight, and whether a save arrived meanwhile.
    checking: Mutex<HashMap<String, bool>>,
    focus: Mutex<Option<FocusTimer>>,
    /// The program started with `run_program`, if any.
    running: Mutex<Option<run::Running>>,
}

impl AppState {
    /// The active profile's quests folder.
    fn quests_dir(&self) -> PathBuf {
        self.quests_root.lock().unwrap().clone()
    }
}

#[derive(Serialize)]
struct QuestDetail {
    quest: Quest,
    lesson: String,
    workspace: Option<String>,
    hints_used: u32,
    attempts: u32,
    completed: bool,
    last_check: Option<CheckResult>,
    /// `run_program` would work: the workspace has a binary / main.lua.
    has_program: bool,
}

#[derive(Serialize)]
struct Home {
    track: Track,
    next: Option<Quest>,
    alternate: Option<Quest>,
    profile: Profile,
    resume: Option<Quest>,
}

#[derive(Serialize)]
struct Achievement {
    id: &'static str,
    title: &'static str,
    description: &'static str,
    hidden: bool,
}

#[derive(Serialize, Clone)]
struct CheckEvent {
    quest_id: String,
    result: CheckResult,
    completion: Option<Completion>,
}

#[derive(Serialize, Clone)]
struct FocusTick {
    remaining_s: u64,
    total_s: u64,
    quest_id: Option<String>,
}

impl AppState {
    fn quest(&self, id: &str) -> CmdResult<&Quest> {
        self.catalog.quest(id).ok_or_else(|| format!("unknown quest `{id}`"))
    }

    /// Profile with the streak adjusted for today.
    fn profile_now(&self) -> CmdResult<Profile> {
        let p = self.progress.lock().unwrap();
        let mut profile = p.profile().map_err(err)?;
        (profile.streak_days, profile.freezes) = p.streak_and_freezes_on(Local::now().date_naive()).map_err(err)?;
        Ok(profile)
    }

    fn detail(&self, id: &str) -> CmdResult<QuestDetail> {
        let quest = self.resolve(self.quest(id)?);
        let lesson = quest.lesson().map_err(err)?;
        let run = self.progress.lock().unwrap().run(id).map_err(err)?;
        let dir = workspace::dir_for(&self.quests_dir(), id);
        let workspace = (quest.has_project() && quest.runner.is_project(&dir))
            .then(|| dir.to_string_lossy().into_owned());
        let has_program = workspace.is_some() && run::has_program(&quest, &dir);
        Ok(QuestDetail {
            quest,
            lesson,
            workspace,
            hints_used: run.as_ref().map(|r| r.hints_used).unwrap_or(0),
            attempts: run.as_ref().map(|r| r.attempts).unwrap_or(0),
            completed: run.as_ref().is_some_and(|r| r.completed_at.is_some()),
            last_check: self.last_checks.lock().unwrap().get(id).cloned(),
            has_program,
        })
    }

    /// The track the learner is on: `settings.track`, else the first one.
    fn active_track(&self) -> CmdResult<&Track> {
        let wanted = self.progress.lock().unwrap().settings().map_err(err)?.track;
        self.catalog
            .track(&wanted)
            .or_else(|| self.catalog.tracks.first())
            .ok_or_else(|| "no tracks in the catalog".to_string())
    }

    /// The quest told in the learner's chosen flavour (plain when it has none).
    fn resolve(&self, q: &Quest) -> Quest {
        let flavour = self.progress.lock().unwrap().settings().map(|s| s.flavour).unwrap_or_default();
        q.resolved(&flavour)
    }

    fn focus_active_for(&self, quest_id: &str) -> bool {
        self.focus
            .lock()
            .unwrap()
            .as_ref()
            .is_some_and(|f| f.quest_id.as_deref().is_none_or(|q| q == quest_id))
    }
}

/// Emit to the UI and log it for the MCP `events` tool.
fn emit<T: Serialize + Clone>(app: &AppHandle, name: &str, payload: T) {
    if let Some(m) = app.try_state::<mcp::Mcp>() {
        m.record(name, serde_json::to_value(&payload).unwrap_or(Value::Null));
    }
    let _ = app.emit(name, payload);
}

fn emit_profile(app: &AppHandle, state: &AppState) {
    if let Ok(p) = state.profile_now() {
        emit(app, "profile:changed", p);
    }
}

/// Run cargo for `quest_id`, award XP on the first green, emit the events. Blocking.
fn do_check(app: &AppHandle, state: &AppState, quest_id: &str) -> CmdResult<(CheckResult, Option<Completion>)> {
    let quest = &state.resolve(state.quest(quest_id)?);
    let dir = workspace::dir_for(&state.quests_dir(), quest_id);
    if !quest.runner.is_project(&dir) {
        return Err("start the quest first".into());
    }
    emit(app, "check:started", serde_json::json!({ "quest_id": quest_id }));

    let result = grader::run_quest(quest, &dir, &state.target_dir).map_err(err)?;
    let now = Local::now();
    let completion = if result.ok {
        let clean = grader::clean_for(quest, &dir, &state.target_dir);
        let focus = state.focus_active_for(quest_id);
        let c = state
            .progress
            .lock()
            .unwrap()
            .complete_quest(&state.catalog, quest, now, focus, clean)
            .map_err(err)?;
        c.first_time.then_some(c)
    } else {
        state.progress.lock().unwrap().record_attempt(quest_id, now).map_err(err)?;
        None
    };

    state.last_checks.lock().unwrap().insert(quest_id.to_string(), result.clone());
    emit(
        app,
        "check:result",
        CheckEvent { quest_id: quest_id.to_string(), result: result.clone(), completion: completion.clone() },
    );
    emit_profile(app, state);
    Ok((result, completion))
}

/// Watcher callback: run a check unless one is running, in which case run again after it.
fn check_from_watcher(app: AppHandle, quest_id: String) {
    std::thread::spawn(move || {
        let state = app.state::<AppState>();
        {
            let mut checking = state.checking.lock().unwrap();
            if let Some(dirty) = checking.get_mut(&quest_id) {
                *dirty = true;
                return;
            }
            checking.insert(quest_id.clone(), false);
        }
        loop {
            if let Err(e) = do_check(&app, &state, &quest_id) {
                log::warn!("check {quest_id}: {e}");
            }
            let mut checking = state.checking.lock().unwrap();
            if checking.get(&quest_id).copied().unwrap_or(false) {
                checking.insert(quest_id.clone(), false);
                continue;
            }
            checking.remove(&quest_id);
            break;
        }
    });
}

fn watch_quest(app: &AppHandle, state: &AppState, quest_id: &str) -> CmdResult<()> {
    let dir = workspace::dir_for(&state.quests_dir(), quest_id);
    let exts = state.quest(quest_id)?.runner.extensions();
    let app2 = app.clone();
    let id = quest_id.to_string();
    let handle = watcher::watch(&dir, Duration::from_millis(400), exts, move || {
        check_from_watcher(app2.clone(), id.clone());
    })
    .map_err(err)?;
    *state.watch.lock().unwrap() = Some((quest_id.to_string(), handle));
    Ok(())
}

// ---------------------------------------------------------------- commands

#[tauri::command]
fn home(state: State<AppState>) -> CmdResult<Home> {
    let profile = state.profile_now()?;
    let track = state.active_track()?.clone();
    let completed: HashSet<String> = profile.completed.iter().cloned().collect();
    let available = state.catalog.available_in(&track.id, &completed);
    let resume = profile
        .started
        .iter()
        .rev()
        .find_map(|id| state.catalog.quest(id))
        .filter(|q| q.track_id == track.id)
        .cloned();
    Ok(Home {
        track,
        next: available.first().map(|q| state.resolve(q)),
        alternate: available.get(1).map(|q| state.resolve(q)),
        profile,
        resume: resume.map(|q| state.resolve(&q)),
    })
}

#[tauri::command]
fn list_tracks(state: State<AppState>) -> Vec<Track> {
    state.catalog.tracks.clone()
}

/// Worlds of one track; `track` omitted = the active track.
#[tauri::command]
fn list_worlds(state: State<AppState>, track: Option<String>) -> CmdResult<Vec<oxide_core::World>> {
    let id = match track {
        Some(t) => t,
        None => state.active_track()?.id.clone(),
    };
    Ok(state
        .catalog
        .worlds_of(&id)
        .map(|w| {
            let mut w = w.clone();
            w.quests = w.quests.iter().map(|q| state.resolve(q)).collect();
            w
        })
        .collect())
}

#[tauri::command]
fn list_flavours(state: State<AppState>) -> Vec<String> {
    state.catalog.flavours()
}

#[tauri::command]
fn get_quest(state: State<AppState>, id: String) -> CmdResult<QuestDetail> {
    state.detail(&id)
}

#[tauri::command]
fn start_quest(app: AppHandle, state: State<AppState>, id: String) -> CmdResult<QuestDetail> {
    let quest = state.resolve(state.quest(&id)?);
    state.progress.lock().unwrap().start_quest(&id, Local::now()).map_err(err)?;
    if quest.has_project() {
        let dir = workspace::start(&state.quests_dir(), &quest).map_err(err)?;
        // The in-app editor is the default; an external editor opens only when configured.
        let editor = state.progress.lock().unwrap().settings().map_err(err)?.editor;
        if !editor.trim().is_empty()
            && let Err(e) = workspace::open_in_editor(&dir, &editor)
        {
            log::warn!("{e:#}");
        }
        watch_quest(&app, &state, &id)?;
        // First check in the background so the objectives show their real state.
        check_from_watcher(app.clone(), id.clone());
    }
    emit_profile(&app, &state);
    state.detail(&id)
}

#[tauri::command]
fn open_editor(app: AppHandle, state: State<AppState>, id: String) -> CmdResult<()> {
    let dir = workspace::dir_for(&state.quests_dir(), &id);
    if !state.quest(&id)?.runner.is_project(&dir) {
        return Err("start the quest first".into());
    }
    let editor = state.progress.lock().unwrap().settings().map_err(err)?.editor;
    if editor.trim().is_empty() {
        return Err("no external editor set: add one (e.g. `code`) in Profile › Settings".into());
    }
    workspace::open_in_editor(&dir, &editor).map_err(err)?;
    let watching = state.watch.lock().unwrap().as_ref().is_some_and(|(q, _)| q == &id);
    if !watching {
        watch_quest(&app, &state, &id)?;
    }
    Ok(())
}

#[tauri::command]
fn reset_quest(app: AppHandle, state: State<AppState>, id: String) -> CmdResult<QuestDetail> {
    let quest = state.resolve(state.quest(&id)?);
    *state.watch.lock().unwrap() = None;
    workspace::reset(&state.quests_dir(), &quest).map_err(err)?;
    state.progress.lock().unwrap().break_combo().map_err(err)?;
    emit_profile(&app, &state);
    state.last_checks.lock().unwrap().remove(&id);
    watch_quest(&app, &state, &id)?;
    state.detail(&id)
}

#[tauri::command]
async fn run_checks(app: AppHandle, state: State<'_, AppState>, id: String) -> CmdResult<CheckResult> {
    let (result, _) = do_check(&app, &state, &id)?;
    Ok(result)
}

#[tauri::command]
fn answer_quiz(app: AppHandle, state: State<AppState>, id: String, answers: Vec<usize>) -> CmdResult<Option<Completion>> {
    let quest = &state.resolve(state.quest(&id)?);
    if quest.kind != QuestKind::Read {
        return Err("not a read quest".into());
    }
    let now = Local::now();
    let p = state.progress.lock().unwrap();
    p.start_quest(&id, now).map_err(err)?;
    let right = answers.len() == quest.quiz.len() && quest.quiz.iter().zip(&answers).all(|(q, a)| q.answer == *a);
    if !right {
        p.record_attempt(&id, now).map_err(err)?;
        drop(p);
        emit_profile(&app, &state);
        return Ok(None);
    }
    let focus = state.focus_active_for(&id);
    let c = p.complete_quest(&state.catalog, quest, now, focus, false).map_err(err)?;
    drop(p);
    emit_profile(&app, &state);
    Ok(Some(c))
}

#[tauri::command]
fn use_hint(state: State<AppState>, id: String) -> CmdResult<u32> {
    let quest = state.quest(&id)?;
    let p = state.progress.lock().unwrap();
    p.start_quest(&id, Local::now()).map_err(err)?;
    p.use_hint(&id, quest.hints.len() as u32).map_err(err)
}

#[tauri::command]
fn read_point(app: AppHandle, state: State<AppState>, id: String, index: u32) -> CmdResult<u32> {
    state.quest(&id)?;
    let xp = state.progress.lock().unwrap().read_point(&id, index).map_err(err)?;
    if xp > 0 {
        emit_profile(&app, &state);
    }
    Ok(xp)
}

#[tauri::command]
fn get_profile(state: State<AppState>) -> CmdResult<Profile> {
    state.profile_now()
}

#[tauri::command]
fn list_achievements() -> Vec<Achievement> {
    ACHIEVEMENTS.iter().map(|a| Achievement { id: a.id, title: a.title, description: a.description, hidden: a.hidden }).collect()
}

#[tauri::command]
fn start_focus(app: AppHandle, state: State<AppState>, id: Option<String>, minutes: u32) -> CmdResult<i64> {
    stop_focus_inner(&state, false)?;
    let session = state.progress.lock().unwrap().start_focus(id.as_deref(), minutes, Local::now()).map_err(err)?;
    let cancel = Arc::new(AtomicBool::new(false));
    *state.focus.lock().unwrap() = Some(FocusTimer { id: session, quest_id: id.clone(), cancel: cancel.clone() });

    let total_s = u64::from(minutes) * 60;
    let app2 = app.clone();
    std::thread::spawn(move || {
        let started = std::time::Instant::now();
        loop {
            if cancel.load(Ordering::Relaxed) {
                return;
            }
            let elapsed = started.elapsed().as_secs();
            let remaining_s = total_s.saturating_sub(elapsed);
            emit(&app2, "focus:tick", FocusTick { remaining_s, total_s, quest_id: id.clone() });
            if remaining_s == 0 {
                let state = app2.state::<AppState>();
                let mut focus = state.focus.lock().unwrap();
                if focus.as_ref().is_some_and(|f| f.id == session) {
                    let _ = state.progress.lock().unwrap().finish_focus(session, true);
                    *focus = None;
                }
                emit(&app2, "focus:done", serde_json::json!({ "quest_id": id }));
                return;
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    });
    Ok(session)
}

fn stop_focus_inner(state: &AppState, completed: bool) -> CmdResult<()> {
    if let Some(f) = state.focus.lock().unwrap().take() {
        f.cancel.store(true, Ordering::Relaxed);
        state.progress.lock().unwrap().finish_focus(f.id, completed).map_err(err)?;
    }
    Ok(())
}

#[tauri::command]
fn stop_focus(state: State<AppState>, completed: bool) -> CmdResult<()> {
    stop_focus_inner(&state, completed)
}

#[tauri::command]
fn run_program(app: AppHandle, state: State<AppState>, id: String) -> CmdResult<()> {
    let quest = state.resolve(state.quest(&id)?);
    let dir = workspace::dir_for(&state.quests_dir(), &id);
    if !quest.runner.is_project(&dir) {
        return Err("start the quest first".into());
    }
    run::start(&app, &state.running, &quest, &dir, &state.target_dir)
}

#[tauri::command]
fn run_input(state: State<AppState>, text: String) -> CmdResult<()> {
    run::input(&state.running, &text)
}

#[tauri::command]
fn stop_program(state: State<AppState>) {
    run::stop(&state.running);
}

/// The started workspace of a quest, for the in-app editor.
fn workspace_of(state: &AppState, id: &str) -> CmdResult<PathBuf> {
    let quest = state.resolve(state.quest(id)?);
    let dir = workspace::dir_for(&state.quests_dir(), id);
    if !dir.join(quest.runner.marker()).exists() {
        return Err("start the quest first".into());
    }
    Ok(dir)
}

#[tauri::command]
fn list_files(state: State<AppState>, id: String) -> CmdResult<Vec<files::FileEntry>> {
    files::list(&workspace_of(&state, &id)?)
}

#[tauri::command]
fn read_file(state: State<AppState>, id: String, path: String) -> CmdResult<String> {
    files::read(&workspace_of(&state, &id)?, &path)
}

/// Writes like a save from any editor: the watcher picks it up and re-grades.
#[tauri::command]
fn write_file(state: State<AppState>, id: String, path: String, text: String) -> CmdResult<()> {
    files::write(&workspace_of(&state, &id)?, &path, &text)
}

#[tauri::command]
fn get_settings(state: State<AppState>) -> CmdResult<Settings> {
    state.progress.lock().unwrap().settings().map_err(err)
}

#[tauri::command]
fn set_settings(app: AppHandle, state: State<AppState>, settings: Settings) -> CmdResult<Settings> {
    state.progress.lock().unwrap().set_settings(&settings).map_err(err)?;
    emit_profile(&app, &state);
    Ok(settings)
}

#[tauri::command]
fn active_dates(state: State<AppState>) -> CmdResult<Vec<String>> {
    state.progress.lock().unwrap().active_dates().map_err(err)
}

// ---------------------------------------------------------------- profiles

#[tauri::command]
fn list_profiles(state: State<AppState>) -> Vec<ProfileInfo> {
    state.profiles.lock().unwrap().list()
}

#[tauri::command]
fn create_profile(state: State<AppState>, name: String) -> CmdResult<Vec<ProfileInfo>> {
    let mut profiles = state.profiles.lock().unwrap();
    profiles.create(&name).map_err(err)?;
    Ok(profiles.list())
}

#[tauri::command]
fn rename_profile(state: State<AppState>, id: String, name: String) -> CmdResult<Vec<ProfileInfo>> {
    let mut profiles = state.profiles.lock().unwrap();
    profiles.rename(&id, &name).map_err(err)?;
    Ok(profiles.list())
}

#[tauri::command]
fn delete_profile(state: State<AppState>, id: String) -> CmdResult<Vec<ProfileInfo>> {
    let mut profiles = state.profiles.lock().unwrap();
    profiles.delete(&id).map_err(err)?;
    Ok(profiles.list())
}

/// Make another profile the active one: its progress and quests folder, from now on. Anything running for
/// the old profile (a focus timer, a program, the file watcher) is stopped first.
#[tauri::command]
fn switch_profile(app: AppHandle, state: State<AppState>, id: String) -> CmdResult<Profile> {
    let (db, quests) = {
        let profiles = state.profiles.lock().unwrap();
        if profiles.active().id == id {
            return state.profile_now();
        }
        (profiles.db_path(&id), profiles.quests_path(&id))
    };
    let progress = Progress::open(&db).map_err(err)?;
    std::fs::create_dir_all(&quests).map_err(err)?;
    stop_focus_inner(&state, false)?;
    run::stop(&state.running);
    *state.watch.lock().unwrap() = None;
    state.last_checks.lock().unwrap().clear();
    *state.progress.lock().unwrap() = progress;
    *state.quests_root.lock().unwrap() = quests;
    state.profiles.lock().unwrap().set_active(&id).map_err(err)?;
    emit_profile(&app, &state);
    state.profile_now()
}

/// Save a profile as one zip in the exports folder and show it in Explorer. Gives back the zip's path.
#[tauri::command]
fn export_profile(app: AppHandle, state: State<AppState>, id: String) -> CmdResult<String> {
    use tauri_plugin_opener::OpenerExt;
    let profiles = state.profiles.lock().unwrap();
    let name = profiles.list().into_iter().find(|p| p.id == id).ok_or("no such profile")?.name;
    let safe: String = name.chars().map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' { c } else { '_' }).collect();
    let dest = state.exports_dir.join(format!("{} {}.oxide-profile.zip", safe.trim(), Local::now().format("%Y-%m-%d %H%M")));
    let progress = state.progress.lock().unwrap();
    let open = (profiles.active().id == id).then_some(&*progress);
    profiles.export(&id, open, &dest).map_err(err)?;
    let _ = app.opener().reveal_item_in_dir(&dest);
    Ok(dest.to_string_lossy().into_owned())
}

/// Add the profile in an exported zip (sent as base64) as a new profile.
#[tauri::command]
fn import_profile(state: State<AppState>, data: String) -> CmdResult<Vec<ProfileInfo>> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD.decode(data.trim()).map_err(err)?;
    let mut profiles = state.profiles.lock().unwrap();
    profiles.import(&bytes).map_err(err)?;
    Ok(profiles.list())
}

// ---------------------------------------------------------------- setup

/// The portable zip's folder: oxide.exe with a `content` folder next to it. Its saves go there too.
fn portable_dir() -> Option<PathBuf> {
    let dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
    dir.join("content").join("tracks").is_dir().then_some(dir)
}

fn content_dir(app: &AppHandle) -> PathBuf {
    if let Some(p) = std::env::var_os("OXIDE_CONTENT") {
        return PathBuf::from(p);
    }
    // Dev builds read the repo, not the resource copy tauri dev made at build time.
    if cfg!(debug_assertions) {
        let repo = oxide_core::repo_content_dir();
        if repo.join("tracks").is_dir() {
            return repo;
        }
    }
    // Portable zip: a `content` folder next to oxide.exe.
    if let Some(dir) = portable_dir() {
        return dir.join("content");
    }
    if let Ok(res) = app.path().resource_dir() {
        let bundled = res.join("content");
        if bundled.join("tracks").is_dir() {
            return bundled;
        }
    }
    oxide_core::repo_content_dir()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    tauri::Builder::default()
        // A second launch focuses the running window instead of opening another instance.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let content = content_dir(&handle);
            let catalog = oxide_core::content::load(&content)
                .map_err(|p| anyhow::anyhow!("content at {}:\n{p}", content.display()))?;
            log::info!("{} worlds, {} quests from {}", catalog.worlds.len(), catalog.quests().count(), content.display());

            // Saves: next to oxide.exe for the portable zip, in AppData otherwise. A save from before profiles
            // (AppData's progress.db and ~/oxide-quests) becomes the first profile, its files left in place.
            let (data_root, legacy, exports_dir) = match portable_dir() {
                Some(dir) => (dir.join("data"), None, dir.join("exports")),
                None => {
                    let app_data = handle.path().app_data_dir()?;
                    let docs = directories::UserDirs::new()
                        .and_then(|u| u.document_dir().map(|d| d.to_path_buf()))
                        .unwrap_or_else(|| app_data.clone());
                    (app_data.clone(), Some((app_data.join("progress.db"), workspace::root())), docs.join("Oxide exports"))
                }
            };
            let target_dir = grader::shared_target_dir(&legacy.as_ref().map_or(data_root.clone(), |(_, quests)| quests.clone()));
            let profiles = Profiles::load(&data_root, legacy)?;
            let active = profiles.active().id.clone();
            let progress = Progress::open(&profiles.db_path(&active))?;
            let quests_root = profiles.quests_path(&active);
            std::fs::create_dir_all(&quests_root)?;
            log::info!("profile {active}: progress {}, quests {}", profiles.db_path(&active).display(), quests_root.display());

            app.manage(mcp::Mcp::default());
            if mcp::enabled()
                && let Err(e) = mcp::start(handle.clone(), mcp::port())
            {
                log::warn!("{e:#}");
            }

            app.manage(AppState {
                catalog,
                progress: Mutex::new(progress),
                quests_root: Mutex::new(quests_root),
                profiles: Mutex::new(profiles),
                exports_dir,
                target_dir,
                watch: Mutex::new(None),
                last_checks: Mutex::new(HashMap::new()),
                checking: Mutex::new(HashMap::new()),
                focus: Mutex::new(None),
                running: Mutex::new(None),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            home,
            list_tracks,
            list_worlds,
            list_flavours,
            get_quest,
            start_quest,
            open_editor,
            reset_quest,
            run_checks,
            answer_quiz,
            use_hint,
            read_point,
            get_profile,
            list_achievements,
            start_focus,
            stop_focus,
            run_program,
            run_input,
            stop_program,
            list_files,
            read_file,
            write_file,
            get_settings,
            set_settings,
            active_dates,
            list_profiles,
            create_profile,
            rename_profile,
            delete_profile,
            switch_profile,
            export_profile,
            import_profile,
            mcp::mcp_reply,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
