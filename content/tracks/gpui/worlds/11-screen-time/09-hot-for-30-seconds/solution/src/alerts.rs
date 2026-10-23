//! Alerts: a toast when a stat stays over its limit for half a minute.

#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use gpui::{App, AppContext, Context, Entity, Task};
use tauri_winrt_notification::Toast;

use crate::stat::ALL;
use crate::*;

/// How long a stat must stay over its limit before Kinetik says so.
pub const HOT_FOR: Duration = Duration::from_secs(30);

/// One alert: which stat, and its value when the alert went off.
#[derive(Debug, Clone, PartialEq)]
pub struct Alert {
    pub kind: StatKind,
    pub value: f64,
}

/// Since when each stat has been over its limit, and which have already alerted.
#[derive(Debug, Default)]
pub struct HotTimer {
    pub over_since: HashMap<StatKind, Instant>,
    pub alerted: HashSet<StatKind>,
}

/// The alerts so far, from the bar's readings.
pub struct Alerts {
    pub timer: HotTimer,
    pub limits: Limits,
    pub fired: Vec<Alert>,
    /// What to do with each new alert: show a toast, or nothing in tests.
    pub notify: fn(&Alert),
    _watch: Task<()>,
}

impl Alerts {
    /// Check every stat once. Each new alert goes to `notify` and into `fired`.
    pub fn check_all(&mut self, snapshot: &Snapshot, now: Instant) {
        for kind in ALL {
            let reading = Reading { kind, value: snapshot.get(&kind).copied() };
            if let Some(alert) = self.timer.check(&reading, &self.limits, now) {
                (self.notify)(&alert);
                self.fired.push(alert);
            }
        }
    }
}

/// Check the bar's readings every second. Not `observe`: the bar only notifies when what it draws changes,
/// and a GPU sitting at 90 °C draws the same thing every second.
pub fn watch_alerts(bar: &Entity<StatsBar>, notify: fn(&Alert), cx: &mut App) -> Entity<Alerts> {
    let bar = bar.downgrade();
    cx.new(|cx| {
        let watch = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                let Ok(snapshot) = bar.read_with(cx, |bar, _| bar.snapshot.clone()) else {
                    break;
                };
                // The executor's clock, not Instant::now: tests move it forward by hand.
                let now = cx.background_executor().now();
                if this.update(cx, |alerts: &mut Alerts, _| alerts.check_all(&snapshot, now)).is_err() {
                    break;
                }
            }
        });
        Alerts { timer: HotTimer::default(), limits: Limits::default(), fired: Vec::new(), notify, _watch: watch }
    })
}
