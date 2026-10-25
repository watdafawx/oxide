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

/// Tier 1 under 100 MB, one more tier every 100 MB.
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

// TODO: a private command `add_download` that takes the shared state and `amount: u32`,
//       locks the state and hands it to add_download_inner

// TODO: a private command `install_mod` that takes the shared state and `id: String`,
//       locks the state and hands it to install_mod_inner

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
    fn add_download_answers_through_the_mock_app() {
        let (_app, webview) = mock_webview();
        let stats = invoke(&webview, "add_download", serde_json::json!({ "amount": 250 }))
            .expect("add_download should be a registered command");
        assert_eq!(stats, serde_json::json!({ "mb": 250, "tier": 3 }));
        let stats = invoke(&webview, "add_download", serde_json::json!({ "amount": 50 })).unwrap();
        assert_eq!(stats["mb"], 300, "the state is shared between calls");
    }

    #[test]
    fn install_mod_answers_and_rejects_repeats() {
        let (_app, webview) = mock_webview();
        let ids = invoke(&webview, "install_mod", serde_json::json!({ "id": "sodium" }))
            .expect("install_mod should be a registered command");
        assert_eq!(ids, serde_json::json!(["sodium"]));
        let again = invoke(&webview, "install_mod", serde_json::json!({ "id": "sodium" }));
        assert!(again.is_err(), "installing the same mod twice must reject");
    }
}
