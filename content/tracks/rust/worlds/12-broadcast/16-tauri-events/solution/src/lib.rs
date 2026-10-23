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
use std::net::{SocketAddr, UdpSocket};
#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use axum::extract::ws::{Message, WebSocket};
#[allow(unused_imports)]
use image::{RgbaImage, imageops};
#[allow(unused_imports)]
use serde_json::Value;
#[allow(unused_imports)]
use tauri::{AppHandle, Emitter, Runtime};
#[allow(unused_imports)]
use tokio::io::AsyncReadExt;
#[allow(unused_imports)]
use tokio::net::TcpStream;
#[allow(unused_imports)]
use tokio::sync::{broadcast, mpsc};

/// The HUD state, for the React pages. Not `pub`: a public command in the crate root clashes with the
/// macro `#[tauri::command]` generates. `build_app` still reaches it as `crate::hud_state`.
#[tauri::command]
fn hud_state(hud: tauri::State<'_, SharedHud>) -> HudState {
    hud.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
}

/// Tell every page about a moment: a `moment` event with its cue and detail.
pub fn publish_moment<R: Runtime>(app: &AppHandle<R>, moment: &Moment) -> tauri::Result<()> {
    app.emit("moment", MomentPayload { cue: moment.cue(), detail: moment.detail() })
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
                url: "http://tauri.localhost".parse().unwrap(),
                body: InvokeBody::Json(serde_json::json!({})),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .expect("the command answers")
        .deserialize::<serde_json::Value>()
        .unwrap()
    }

    #[test]
    fn moment_event_emitted_once() {
        use tauri::Listener;
        let app = mock_app();
        let got = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
        let sink = got.clone();
        app.listen_any("moment", move |event| sink.lock().unwrap().push(event.payload().to_string()));
        publish_moment(app.handle(), &Moment::HatTrick("Wren".into())).unwrap();
        assert_eq!(got.lock().unwrap().as_slice(), [r#"{"cue":"hat_trick","detail":"Wren"}"#]);
    }
}
