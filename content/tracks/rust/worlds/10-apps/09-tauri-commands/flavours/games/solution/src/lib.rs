use std::sync::Mutex;

use serde::Serialize;

#[derive(Debug, Default)]
pub struct AppState {
    pub downloaded_mb: u32,
    pub installed: Vec<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct PackStats {
    pub mb: u32,
    pub tier: u32,
}

/// Tier 1 at 0 MB downloaded, one more tier every 100 MB.
pub fn tier_for(mb: u32) -> u32 {
    mb / 100 + 1
}

/// Adds `amount` MB to the state and returns the new stats.
pub fn add_download_inner(state: &mut AppState, amount: u32) -> PackStats {
    state.downloaded_mb += amount;
    PackStats {
        mb: state.downloaded_mb,
        tier: tier_for(state.downloaded_mb),
    }
}

/// Records an installed mod and returns every installed id.
/// Blank ids and ids that were already installed are errors.
pub fn install_mod_inner(state: &mut AppState, id: String) -> Result<Vec<String>, String> {
    if id.trim().is_empty() {
        return Err("mod id is blank".into());
    }
    if state.installed.contains(&id) {
        return Err(format!("{id} is already installed"));
    }
    state.installed.push(id);
    Ok(state.installed.clone())
}

// (private on purpose: a pub command in the crate root clashes with the macro Tauri generates)
#[tauri::command]
fn add_download(state: tauri::State<'_, Mutex<AppState>>, amount: u32) -> PackStats {
    add_download_inner(&mut state.lock().unwrap(), amount)
}

// (private on purpose: a pub command in the crate root clashes with the macro Tauri generates)
#[tauri::command]
fn install_mod(
    state: tauri::State<'_, Mutex<AppState>>,
    id: String,
) -> Result<Vec<String>, String> {
    install_mod_inner(&mut state.lock().unwrap(), id)
}

/// Registers the state and both commands on any Tauri builder (real or mock).
pub fn build_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![add_download, install_mod])
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::ipc::{CallbackFn, InvokeBody};
    use tauri::test::{INVOKE_KEY, get_ipc_response, mock_builder, mock_context, noop_assets};
    use tauri::webview::InvokeRequest;

    #[test]
    fn tier_rises_every_100_mb() {
        assert_eq!(tier_for(0), 1);
        assert_eq!(tier_for(99), 1);
        assert_eq!(tier_for(100), 2);
        assert_eq!(tier_for(250), 3);
    }

    #[test]
    fn add_download_adds_and_reports_the_tier() {
        let mut state = AppState::default();
        assert_eq!(add_download_inner(&mut state, 60), PackStats { mb: 60, tier: 1 });
        assert_eq!(add_download_inner(&mut state, 60), PackStats { mb: 120, tier: 2 });
        assert_eq!(state.downloaded_mb, 120);
    }

    #[test]
    fn install_mod_rejects_blank_and_duplicate_ids() {
        let mut state = AppState::default();
        let ids = install_mod_inner(&mut state, "sodium".into()).unwrap();
        assert_eq!(ids, vec!["sodium".to_string()]);
        assert!(install_mod_inner(&mut state, "sodium".into()).is_err());
        assert!(install_mod_inner(&mut state, "  ".into()).is_err());
        assert_eq!(state.installed.len(), 1, "failed calls must not change the state");
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

        let stats = invoke(&webview, "add_download", serde_json::json!({ "amount": 250 })).unwrap();
        assert_eq!(stats["mb"], 250);
        assert_eq!(stats["tier"], 3);

        let ids = invoke(&webview, "install_mod", serde_json::json!({ "id": "sodium" })).unwrap();
        assert_eq!(ids, serde_json::json!(["sodium"]));

        let err = invoke(&webview, "install_mod", serde_json::json!({ "id": "sodium" }));
        assert!(err.is_err(), "installing twice must reject");
    }
}
