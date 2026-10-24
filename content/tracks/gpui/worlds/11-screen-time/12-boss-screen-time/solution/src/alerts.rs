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

impl HotTimer {
    /// Look at one reading taken at `now`. An alert the first time it has been over its limit for 30 seconds;
    /// back under the limit starts the wait again.
    pub fn check(&mut self, reading: &Reading, limits: &Limits, now: Instant) -> Option<Alert> {
        let kind = reading.kind;
        if !is_warning(reading, limits) {
            self.over_since.remove(&kind);
            self.alerted.remove(&kind);
            return None;
        }
        let since = *self.over_since.entry(kind).or_insert(now);
        if now.duration_since(since) >= HOT_FOR && self.alerted.insert(kind) {
            return reading.value.map(|value| Alert { kind, value });
        }
        None
    }
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

/// The toast's title and text for an alert.
pub fn toast_text(alert: &Alert) -> (String, String) {
    let title = match alert.kind {
        StatKind::CpuTemp | StatKind::GpuTemp => format!("{} is hot", alert.kind.label()),
        StatKind::CpuUsage | StatKind::GpuUsage => format!("{} is busy", alert.kind.label()),
        StatKind::Ping => "Ping is high".to_string(),
        StatKind::Battery => "Battery is low".to_string(),
        StatKind::DiskFree => "C: is nearly full".to_string(),
        kind => format!("{} warning", kind.label()),
    };
    let now = Reading { kind: alert.kind, value: Some(alert.value) }.text();
    (title, format!("{now} for 30 seconds or more."))
}

/// Show a Windows toast for an alert. A toast that fails to show is not worth stopping for.
pub fn show_toast(alert: &Alert) {
    let (title, text) = toast_text(alert);
    let _ = Toast::new(Toast::POWERSHELL_APP_ID).title(&title).text1(&text).show();
}
