//! Re-grade when the learner saves. Debounced so one Ctrl+S = one run.

use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use notify::RecursiveMode;
use notify_debouncer_mini::{DebounceEventResult, Debouncer, new_debouncer};

/// Keep the returned value alive for as long as you want events.
pub type Handle = Debouncer<notify::RecommendedWatcher>;

/// Call `on_change` (on a background thread) after `debounce` of quiet following
/// a write to a file with one of `extensions` under `dir`, ignoring `target/` and `build/`.
pub fn watch(
    dir: &Path,
    debounce: Duration,
    extensions: &'static [&'static str],
    on_change: impl Fn() + Send + 'static,
) -> Result<Handle> {
    let mut debouncer = new_debouncer(debounce, move |res: DebounceEventResult| {
        if let Ok(events) = res {
            let relevant = events.iter().any(|e| {
                !e.path.components().any(|c| c.as_os_str() == "target" || c.as_os_str() == "build" || c.as_os_str() == ".gradle")
                    && e.path.extension().and_then(|x| x.to_str()).is_some_and(|x| extensions.contains(&x))
            });
            if relevant {
                on_change();
            }
        }
    })?;
    debouncer.watcher().watch(dir, RecursiveMode::Recursive)?;
    Ok(debouncer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn fires_on_rs_write() {
        let tmp = tempfile::tempdir().unwrap();
        let (tx, rx) = mpsc::channel();
        let _h = watch(tmp.path(), Duration::from_millis(100), &["rs"], move || {
            let _ = tx.send(());
        })
        .unwrap();
        std::thread::sleep(Duration::from_millis(200));
        std::fs::write(tmp.path().join("main.rs"), "fn main(){}").unwrap();
        rx.recv_timeout(Duration::from_secs(5)).expect("watcher should fire");
    }
}
