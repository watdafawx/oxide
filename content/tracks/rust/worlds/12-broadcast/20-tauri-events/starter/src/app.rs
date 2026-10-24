//! The Tauri shell: the React HUD pages call Rust commands and listen for Rust events.

#![allow(unused_imports)]

use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Runtime};

use crate::*;

/// The HUD state, as Tauri manages it for every command.
pub type SharedHud = Mutex<HudState>;

/// The app: the shared HUD state and the commands the pages may call.
pub fn build_app<R: Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder.manage(SharedHud::default()).invoke_handler(tauri::generate_handler![crate::hud_state])
}

/// What a page gets with each `moment` event.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MomentPayload {
    pub cue: &'static str,
    pub detail: String,
}
