//! Tauri commands and events over `oxide_core`. The contract is `docs/API.md`.

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
    quests_root: PathBuf,
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
        profile.streak_days = p.streak_on(Local::now().date_naive()).map_err(err)?;
        Ok(profile)
    }

    fn detail(&self, id: &str) -> CmdResult<QuestDetail> {
        let quest = self.resolve(self.quest(id)?);
        let lesson = quest.lesson().map_err(err)?;
        let run = self.progress.lock().unwrap().run(id).map_err(err)?;
        let dir = workspace::dir_for(&self.quests_root, id);
        let workspace = (quest.has_project() && dir.join(quest.runner.marker()).exists())
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
    let dir = workspace::dir_for(&state.quests_root, quest_id);
    if !dir.join(quest.runner.marker()).exists() {
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
    let dir = workspace::dir_for(&state.quests_root, quest_id);
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
        let dir = workspace::start(&state.quests_root, &quest).map_err(err)?;
        let editor = state.progress.lock().unwrap().settings().map_err(err)?.editor;
        if let Err(e) = workspace::open_in_editor(&dir, &editor) {
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
    let dir = workspace::dir_for(&state.quests_root, &id);
    if !dir.join(state.quest(&id)?.runner.marker()).exists() {
        return Err("start the quest first".into());
    }
    let editor = state.progress.lock().unwrap().settings().map_err(err)?.editor;
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
    workspace::reset(&state.quests_root, &quest).map_err(err)?;
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
fn get_profile(state: State<AppState>) -> CmdResult<Profile> {
    state.profile_now()
}

#[tauri::command]
fn list_achievements() -> Vec<Achievement> {
    ACHIEVEMENTS.iter().map(|(id, title, description)| Achievement { id, title, description }).collect()
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
    let dir = workspace::dir_for(&state.quests_root, &id);
    if !dir.join(quest.runner.marker()).exists() {
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

// ---------------------------------------------------------------- setup

fn content_dir(app: &AppHandle) -> PathBuf {
    if let Some(p) = std::env::var_os("OXIDE_CONTENT") {
        return PathBuf::from(p);
    }
    // Dev builds read the repo, not the resource copy tauri dev made at build time.
    if cfg!(debug_assertions) {
        let repo = oxide_core::repo_content_dir();
        if repo.join("worlds").is_dir() {
            return repo;
        }
    }
    if let Ok(res) = app.path().resource_dir() {
        let bundled = res.join("content");
        if bundled.join("worlds").is_dir() {
            return bundled;
        }
    }
    oxide_core::repo_content_dir()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let content = content_dir(&handle);
            let catalog = oxide_core::content::load(&content)
                .map_err(|p| anyhow::anyhow!("content at {}:\n{p}", content.display()))?;
            log::info!("{} worlds, {} quests from {}", catalog.worlds.len(), catalog.quests().count(), content.display());

            let db = handle.path().app_data_dir()?.join("progress.db");
            let progress = Progress::open(&db)?;
            let quests_root = workspace::root();
            std::fs::create_dir_all(&quests_root)?;
            let target_dir = grader::shared_target_dir(&quests_root);

            app.manage(mcp::Mcp::default());
            if mcp::enabled()
                && let Err(e) = mcp::start(handle.clone(), mcp::port())
            {
                log::warn!("{e:#}");
            }

            app.manage(AppState {
                catalog,
                progress: Mutex::new(progress),
                quests_root,
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
            get_profile,
            list_achievements,
            start_focus,
            stop_focus,
            run_program,
            run_input,
            stop_program,
            get_settings,
            set_settings,
            active_dates,
            mcp::mcp_reply,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
