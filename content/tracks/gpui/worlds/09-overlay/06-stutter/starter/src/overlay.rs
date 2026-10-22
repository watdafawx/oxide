//! The game overlay: games, the strip window, hotkeys, auto-show, sessions and benchmarks.

#![allow(unused_imports)]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use global_hotkey::GlobalHotKeyEvent;
use global_hotkey::HotKeyState;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use gpui::{App, Bounds, Context, Div, IntoElement, PathBuilder, Pixels, Render, Task, Window, WindowBackgroundAppearance, WindowBounds, WindowHandle, WindowKind, WindowOptions, actions, canvas, div, point, prelude::*, px, rgb, rgba, size};

use crate::*;
