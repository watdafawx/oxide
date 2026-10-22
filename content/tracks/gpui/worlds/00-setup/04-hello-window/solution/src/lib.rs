//! Kinetik Stats, rebuilt in Rust.

use gpui::{Pixels, Size, px, size};

/// The name on Kinetik's window.
pub fn app_title() -> String {
    "Kinetik Stats".to_string()
}

/// How big Kinetik's first window is.
pub fn bar_size() -> Size<Pixels> {
    size(px(360.), px(120.))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_is_360_by_120() {
        assert_eq!(bar_size(), size(px(360.), px(120.)));
    }
}
