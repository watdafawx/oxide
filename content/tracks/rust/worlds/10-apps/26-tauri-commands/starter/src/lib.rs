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
    xp / 100 + 1
}

/// Adds `amount` XP to the state and returns the new profile.
pub fn award_xp_inner(state: &mut AppState, amount: u32) -> Profile {
    state.xp += amount;
    Profile {
        xp: state.xp,
        level: level_for(state.xp),
    }
}

/// Records a started quest and returns every started id.
/// Blank ids and ids that were already started are errors.
pub fn start_quest_inner(state: &mut AppState, id: String) -> Result<Vec<String>, String> {
    if id.trim().is_empty() {
        return Err("quest id is blank".into());
    }
    if state.started.contains(&id) {
        return Err(format!("{id} was already started"));
    }
    state.started.push(id);
    Ok(state.started.clone())
}

// TODO: a private command `award_xp` that takes the shared state and `amount: u32`,
//       locks the state and hands it to award_xp_inner

// TODO: a private command `start_quest` that takes the shared state and `id: String`,
//       locks the state and hands it to start_quest_inner

/// Registers the shared state and both commands on any Tauri builder (real or mock).
pub fn build_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    // TODO: hand Tauri a Mutex around a default AppState, and register both commands
    builder
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::ipc::{CallbackFn, InvokeBody};
    use tauri::test::{INVOKE_KEY, get_ipc_response, mock_builder, mock_context, noop_assets};
    use tauri::webview::InvokeRequest;

    /// Builds a mock app (no real window) and its one webview. Keep the app alive
    /// for as long as the webview is used.
    fn mock_webview() -> (
        tauri::App<tauri::test::MockRuntime>,
        tauri::WebviewWindow<tauri::test::MockRuntime>,
    ) {
        let app = build_app(mock_builder())
            .build(mock_context(noop_assets()))
            .expect("mock app should build");
        let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .unwrap();
        (app, webview)
    }

    /// Sends one command, the same way the page's `invoke` does.
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
                // The app's own pages: tauri://localhost, or http://tauri.localhost on Windows.
                url: if cfg!(windows) { "http://tauri.localhost" } else { "tauri://localhost" }
                    .parse()
                    .unwrap(),
                body: InvokeBody::Json(body),
                headers: Default::default(),
                invoke_key: INVOKE_KEY.to_string(),
            },
        )
        .map(|b| b.deserialize::<serde_json::Value>().unwrap())
    }

    #[test]
    fn award_xp_answers_through_the_mock_app() {
        let (_app, webview) = mock_webview();
        let profile = invoke(&webview, "award_xp", serde_json::json!({ "amount": 250 }))
            .expect("award_xp should be a registered command");
        assert_eq!(profile, serde_json::json!({ "xp": 250, "level": 3 }));
        let profile = invoke(&webview, "award_xp", serde_json::json!({ "amount": 50 })).unwrap();
        assert_eq!(profile["xp"], 300, "the state is shared between calls");
    }

    #[test]
    fn start_quest_answers_and_rejects_repeats() {
        let (_app, webview) = mock_webview();
        let ids = invoke(&webview, "start_quest", serde_json::json!({ "id": "01-box" }))
            .expect("start_quest should be a registered command");
        assert_eq!(ids, serde_json::json!(["01-box"]));
        let again = invoke(&webview, "start_quest", serde_json::json!({ "id": "01-box" }));
        assert!(again.is_err(), "starting the same quest twice must reject");
    }
}
