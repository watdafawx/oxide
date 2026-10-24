use std::sync::Mutex;

use serde::Serialize;

#[derive(Debug, Default)]
pub struct AppState {
    pub xp: u32,
    pub started: Vec<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Profile {
    pub xp: u32,
    pub level: u32,
}

/// Level 1 at 0 XP, one more level every 100 XP.
pub fn level_for(xp: u32) -> u32 {
    // TODO: one level per full 100 XP, starting at level 1 (integer division)
    todo!()
}

/// Adds `amount` XP to the state and returns the new profile.
pub fn award_xp_inner(state: &mut AppState, amount: u32) -> Profile {
    // TODO: add amount to the stored XP, then return a Profile with the new XP and its level
    todo!()
}

/// Records a started quest and returns every started id.
/// Blank ids and ids that were already started are errors.
pub fn start_quest_inner(state: &mut AppState, id: String) -> Result<Vec<String>, String> {
    // TODO: reject a blank id and an id that was already started (both Err) BEFORE changing
    //       the state; otherwise record it and return a copy of every started id
    todo!()
}

// (private on purpose: a pub command in the crate root clashes with the macro Tauri generates)
#[tauri::command]
fn award_xp(state: tauri::State<'_, Mutex<AppState>>, amount: u32) -> Profile {
    // TODO: lock the shared state and hand it to award_xp_inner
    todo!()
}

// (private on purpose: a pub command in the crate root clashes with the macro Tauri generates)
#[tauri::command]
fn start_quest(
    state: tauri::State<'_, Mutex<AppState>>,
    id: String,
) -> Result<Vec<String>, String> {
    // TODO: lock the shared state and hand it to start_quest_inner
    todo!()
}

/// Registers the state and both commands on any Tauri builder (real or mock).
pub fn build_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![award_xp, start_quest])
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::ipc::{CallbackFn, InvokeBody};
    use tauri::test::{INVOKE_KEY, get_ipc_response, mock_builder, mock_context, noop_assets};
    use tauri::webview::InvokeRequest;

    #[test]
    fn level_rises_every_100_xp() {
        assert_eq!(level_for(0), 1);
        assert_eq!(level_for(99), 1);
        assert_eq!(level_for(100), 2);
        assert_eq!(level_for(250), 3);
    }

    #[test]
    fn award_xp_adds_and_reports_level() {
        let mut state = AppState::default();
        assert_eq!(award_xp_inner(&mut state, 60), Profile { xp: 60, level: 1 });
        assert_eq!(award_xp_inner(&mut state, 60), Profile { xp: 120, level: 2 });
        assert_eq!(state.xp, 120);
    }

    #[test]
    fn start_quest_rejects_blank_and_duplicate_ids() {
        let mut state = AppState::default();
        let ids = start_quest_inner(&mut state, "01-box".into()).unwrap();
        assert_eq!(ids, vec!["01-box".to_string()]);
        assert!(start_quest_inner(&mut state, "01-box".into()).is_err());
        assert!(start_quest_inner(&mut state, "  ".into()).is_err());
        assert_eq!(state.started.len(), 1, "failed calls must not change the state");
    }

    fn invoke(
        webview: &tauri::WebviewWindow<tauri::test::MockRuntime>,
        cmd: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, serde_json::Value> {
        get_ipc_response(
            webview,
            InvokeRequest {
                cmd: cmd.into(),
                callback: CallbackFn(0),
                error: CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(),
                body: InvokeBody::Json(body),
                headers: Default::default(),
                invoke_key: INVOKE_KEY.to_string(),
            },
        )
        .map(|b| b.deserialize::<serde_json::Value>().unwrap())
    }

    #[test]
    fn commands_work_through_the_mock_app() {
        let app = build_app(mock_builder())
            .build(mock_context(noop_assets()))
            .expect("mock app should build");
        let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .unwrap();

        let profile = invoke(&webview, "award_xp", serde_json::json!({ "amount": 250 })).unwrap();
        assert_eq!(profile["xp"], 250);
        assert_eq!(profile["level"], 3);

        let ids = invoke(&webview, "start_quest", serde_json::json!({ "id": "01-box" })).unwrap();
        assert_eq!(ids, serde_json::json!(["01-box"]));

        let err = invoke(&webview, "start_quest", serde_json::json!({ "id": "01-box" }));
        assert!(err.is_err(), "starting twice must reject");
    }
}
