//! The bar's window.

use gpui::{Bounds, Pixels, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions};

/// The window the bar lives in: no title bar, a pop-up that stays out of the taskbar,
/// not movable or resizable, never taking focus, see-through where nothing is drawn.
pub fn bar_window_options(bounds: Bounds<Pixels>) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: None,
        kind: WindowKind::PopUp,
        is_movable: false,
        is_resizable: false,
        is_minimizable: false,
        focus: false,
        window_background: WindowBackgroundAppearance::Transparent,
        ..Default::default()
    }
}
