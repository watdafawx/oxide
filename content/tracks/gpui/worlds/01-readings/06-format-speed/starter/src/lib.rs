//! Kinetik's stat model.

pub mod format;
pub mod reading;
pub mod stat;

pub use format::{format_bytes, format_percent};
pub use reading::Reading;
pub use stat::{ALL, StatKind, Unit};

/// A network speed: "↓ 3.4 MB/s".
pub fn format_speed(arrow: &str, bytes_per_sec: f64) -> String {
    let _ = (arrow, bytes_per_sec);
    String::new() // TODO: the arrow, a space, the speed written by format_bytes, then "/s"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_shows_down_arrow() {
        let r = Reading { kind: StatKind::NetDown, value: Some(3_565_158.0) };
        assert_eq!(r.text(), "↓ 3.4 MB/s");
    }

    #[test]
    fn upload_shows_up_arrow() {
        let r = Reading { kind: StatKind::NetUp, value: Some(52_000.0) };
        assert_eq!(r.text(), "↑ 50.8 KB/s");
    }
}
