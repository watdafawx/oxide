//! hud: a Rocket League broadcast HUD server.

pub mod frames;
pub mod event;
pub mod feed;
pub mod dedup;
pub mod grace;
pub mod moments;
pub mod triggers;
pub mod osc;
pub mod overlay;
pub mod roster;
pub mod names;
pub mod cuts;
pub mod blocking;
pub mod app;
pub mod pipeline;
pub mod replay;

#[allow(unused_imports)]
pub use frames::*;
#[allow(unused_imports)]
pub use event::*;
#[allow(unused_imports)]
pub use feed::*;
#[allow(unused_imports)]
pub use dedup::*;
#[allow(unused_imports)]
pub use grace::*;
#[allow(unused_imports)]
pub use moments::*;
#[allow(unused_imports)]
pub use triggers::*;
#[allow(unused_imports)]
pub use osc::*;
#[allow(unused_imports)]
pub use overlay::*;
#[allow(unused_imports)]
pub use roster::*;
#[allow(unused_imports)]
pub use names::*;
#[allow(unused_imports)]
pub use cuts::*;
#[allow(unused_imports)]
pub use blocking::*;
#[allow(unused_imports)]
pub use app::*;
#[allow(unused_imports)]
pub use pipeline::*;
#[allow(unused_imports)]
pub use replay::*;

#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use image::RgbaImage;
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use tokio::sync::mpsc;

/// The HUD state, for the React pages. Not `pub`: a public command in the crate root clashes with the
/// macro `#[tauri::command]` generates. `build_app` still reaches it as `crate::hud_state`.
#[tauri::command]
fn hud_state(hud: tauri::State<'_, SharedHud>) -> HudState {
    hud.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
}

impl Pipeline {
    /// One event at `now`: update the HUD; then, after the grace period and unless it is a repeated goal,
    /// the goal or match end, then any moments, each with its triggers' outputs.
    pub fn step(&mut self, event: &GameEvent, now: Duration) -> Vec<Action> {
        let _ = (event, now);
        // TODO: HUD first; then grace and dedup; then the goal or match end, then moments, each with its outputs
        Vec::new()
    }
}

#[cfg(test)]
#[allow(unused_imports, dead_code, unused_variables, unused_mut)]
mod tests {
    use super::*;
    use std::time::Duration;

    const MATCH: &str = include_str!("../fixtures/match.jsonl");

    /// The recorded match's frames, without their times.
    fn frames() -> Vec<serde_json::Value> {
        MATCH.lines().map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap()["frame"].clone()).collect()
    }

    /// The recorded match's frames, back to back, as the Stats API sends them.
    fn wire() -> Vec<u8> {
        frames().iter().flat_map(|f| serde_json::to_vec(f).unwrap()).collect()
    }

    /// Every event in the recorded match, with its time.
    fn events() -> Vec<(Duration, GameEvent)> {
        MATCH
            .lines()
            .filter_map(|line| {
                let v: serde_json::Value = serde_json::from_str(line).unwrap();
                normalize(v["frame"].clone()).map(|e| (Duration::from_millis(v["t_ms"].as_u64().unwrap()), e))
            })
            .collect()
    }

    fn goal(scorer: &str, team_num: u8, goal_time: u32) -> Goal {
        Goal { scorer: PlayerRef { name: scorer.into(), team_num }, assister: None, goal_time }
    }

    fn secs(s: f64) -> Duration {
        Duration::from_secs_f64(s)
    }

    fn triggers() -> Vec<Trigger> {
        load_triggers(include_str!("../fixtures/triggers.json")).unwrap()
    }

    /// A 100 x 60 transparent picture with a solid 30 x 20 block at (20, 10).
    fn photo() -> RgbaImage {
        let mut img = RgbaImage::new(100, 60);
        for x in 20..50 {
            for y in 10..30 {
                img.put_pixel(x, y, image::Rgba([200, 40, 40, 255]));
            }
        }
        img
    }

    /// A fresh, empty folder for one test.
    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("hud-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn mock_app() -> tauri::App<tauri::test::MockRuntime> {
        build_app(tauri::test::mock_builder()).build(tauri::test::mock_context(tauri::test::noop_assets())).expect("the mock app builds")
    }

    /// Call a command the way a page would, and get its JSON answer.
    fn invoke(app: &tauri::App<tauri::test::MockRuntime>, cmd: &str) -> serde_json::Value {
        use tauri::ipc::{CallbackFn, InvokeBody};
        use tauri::webview::InvokeRequest;
        let webview = tauri::WebviewWindowBuilder::new(app, "main", Default::default()).build().unwrap();
        tauri::test::get_ipc_response(
            &webview,
            InvokeRequest {
                cmd: cmd.into(),
                callback: CallbackFn(0),
                error: CallbackFn(1),
                // The app's own pages: tauri://localhost, or http://tauri.localhost on Windows.
                url: if cfg!(windows) { "http://tauri.localhost" } else { "tauri://localhost" }.parse().unwrap(),
                body: InvokeBody::Json(serde_json::json!({})),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .expect("the command answers")
        .deserialize::<serde_json::Value>()
        .unwrap()
    }

    fn golden_lines() -> Vec<String> {
        include_str!("../fixtures/match.golden").lines().map(String::from).collect()
    }

    fn run_match() -> (Pipeline, Vec<String>) {
        let mut pipeline = Pipeline::new(triggers());
        pipeline.connected(Duration::ZERO);
        let mut lines = Vec::new();
        for (t, event) in events() {
            lines.extend(pipeline.step(&event, t).iter().map(Action::line));
        }
        (pipeline, lines)
    }

    #[tokio::test]
    async fn parses_feed() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            let (mut socket, _) = listener.accept().await.unwrap();
            socket.write_all(&wire()).await.unwrap();
        });
        let (tx, mut rx) = mpsc::channel(1024);
        tokio::spawn(run_feed(addr, Duration::from_secs(10), tx));
        let (mut goals, mut updates) = (0, 0);
        while let Ok(Some(event)) = tokio::time::timeout(Duration::from_secs(2), rx.recv()).await {
            match event {
                GameEvent::GoalScored(_) => goals += 1,
                GameEvent::UpdateState(_) => updates += 1,
                _ => {}
            }
        }
        assert_eq!((goals, updates), (9, 322));
    }

    #[test]
    fn goal_dedup() {
        let (_, lines) = run_match();
        let goals: Vec<&String> = lines.iter().filter(|l| l.contains(" goal ")).collect();
        assert_eq!(goals.len(), 7, "the repeat and the stale goal before the match do not fire: {goals:#?}");
        assert!(goals[0].trim_start().starts_with("20000 goal Wren"));
    }

    #[test]
    fn moments_fire() {
        let (_, lines) = run_match();
        for want in [" 210000 hat_trick Wren", " 230000 demo blue 3", " 270000 thirty_seconds", " 301000 overtime"] {
            assert!(lines.iter().any(|l| l.starts_with(want)), "no line starting {want:?} in {lines:#?}");
        }
    }

    #[test]
    fn triggers_sent() {
        let (_, lines) = run_match();
        assert!(lines.contains(&"  20000 goal Wren -> companion 1/0/3".to_string()));
        assert!(lines.contains(&" 321000 match_ended orange -> companion 1/0/4".to_string()));
        assert!(lines.contains(&" 270000 thirty_seconds -> osc /notch/30s".to_string()));
    }

    #[test]
    fn overlay_updates() {
        let (pipeline, _) = run_match();
        assert_eq!(pipeline.hud, HudState { blue: 3, orange: 4, time_seconds: 21, overtime: true, last_goal: Some("Pike".into()) });
    }

    #[test]
    fn tauri_command_live() {
        use tauri::Manager;
        let (pipeline, _) = run_match();
        let app = mock_app();
        *app.state::<SharedHud>().lock().unwrap() = pipeline.hud.clone();
        let state = invoke(&app, "hud_state");
        assert_eq!((state["blue"].as_u64(), state["orange"].as_u64(), state["last_goal"].as_str()), (Some(3), Some(4), Some("Pike")));
    }

    #[test]
    fn replay_identical() {
        let now = replay(MATCH, triggers()).unwrap();
        assert_eq!(diff_lines(&golden_lines(), &now), Vec::<String>::new());
    }
}
