//! Screen time: which program is in front, counted in 5-second samples, one file per day.

#![allow(unused_imports)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{NaiveDate, NaiveDateTime, Timelike};
use gpui::{Context, Task};
use serde::{Deserialize, Serialize};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::StationsAndDesktops::{CloseDesktop, DESKTOP_CONTROL_FLAGS, DESKTOP_SWITCHDESKTOP, OpenInputDesktop};
use windows::Win32::System::SystemInformation::GetTickCount;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use windows::core::PWSTR;

use crate::*;

/// What screen time asks Windows. `RealDesk` asks; tests use a `FakeDesk`.
pub trait Desk {
    /// The full path of the program whose window is in front, like `C:\Windows\notepad.exe`.
    fn foreground_exe(&self) -> Option<String>;
    /// How long since the last key press or mouse move.
    fn idle(&self) -> Duration;
    /// Does the window in front cover its whole monitor, like a video or a game?
    fn fullscreen(&self) -> bool;
    /// Is the PC locked?
    fn locked(&self) -> bool;
    /// The local date and time.
    fn now(&self) -> NaiveDateTime;
}

/// A desk set up by hand, for tests.
#[derive(Debug, Clone)]
pub struct FakeDesk {
    pub exe: Option<String>,
    pub idle: Duration,
    pub fullscreen: bool,
    pub locked: bool,
    pub now: NaiveDateTime,
}

impl FakeDesk {
    /// `exe` in front at `now`: someone at the keyboard, not full screen, not locked.
    pub fn new(exe: &str, now: NaiveDateTime) -> Self {
        FakeDesk { exe: Some(exe.to_string()), idle: Duration::ZERO, fullscreen: false, locked: false, now }
    }
}

impl Desk for FakeDesk {
    fn foreground_exe(&self) -> Option<String> {
        self.exe.clone()
    }

    fn idle(&self) -> Duration {
        self.idle
    }

    fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    fn locked(&self) -> bool {
        self.locked
    }

    fn now(&self) -> NaiveDateTime {
        self.now
    }
}

/// Asks Windows.
#[derive(Debug, Default, Clone, Copy)]
pub struct RealDesk;

impl Desk for RealDesk {
    fn foreground_exe(&self) -> Option<String> {
        // SAFETY: the window and process handles are checked before use, and the process handle is closed.
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.is_invalid() {
                return None;
            }
            let mut pid = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
            let mut buffer = [0u16; 1024];
            let mut len = buffer.len() as u32;
            let read = QueryFullProcessImageNameW(process, PROCESS_NAME_WIN32, PWSTR(buffer.as_mut_ptr()), &mut len);
            let _ = CloseHandle(process);
            read.ok()?;
            Some(String::from_utf16_lossy(&buffer[..len as usize]))
        }
    }

    fn idle(&self) -> Duration {
        let mut info = LASTINPUTINFO { cbSize: size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };
        // SAFETY: `info` is a LASTINPUTINFO with its size filled in, as the function asks.
        if !unsafe { GetLastInputInfo(&mut info) }.as_bool() {
            return Duration::ZERO;
        }
        // SAFETY: GetTickCount has no inputs. Both are milliseconds since boot, wrapping after 49 days.
        let now = unsafe { GetTickCount() };
        Duration::from_millis(u64::from(now.wrapping_sub(info.dwTime)))
    }

    fn fullscreen(&self) -> bool {
        RealWin.foreground().is_some_and(|(window, monitor)| covers_monitor(window, monitor))
    }

    fn locked(&self) -> bool {
        // While the lock screen is up, the desktop that gets input is the secure one, and opening it fails.
        // SAFETY: the desktop handle is closed straight away.
        match unsafe { OpenInputDesktop(DESKTOP_CONTROL_FLAGS(0), false, DESKTOP_SWITCHDESKTOP) } {
            Ok(desktop) => {
                let _ = unsafe { CloseDesktop(desktop) };
                false
            }
            Err(_) => true,
        }
    }

    fn now(&self) -> NaiveDateTime {
        chrono::Local::now().naive_local()
    }
}

/// The program's file name from its full path, in lower case: `C:\Games\Minecraft.Windows.exe` gives `minecraft.windows.exe`.
pub fn exe_name(path: &str) -> String {
    path.rsplit(['\\', '/']).next().unwrap_or(path).to_lowercase()
}

/// The program to count for this sample, if any: nothing while the PC is locked or nobody is using it.
pub fn app_to_count(desk: &dyn Desk) -> Option<String> {
    if desk.locked() || away(desk.idle(), desk.fullscreen()) {
        return None;
    }
    desk.foreground_exe().map(|path| exe_name(&path))
}

/// After this long without a key press or mouse move, nobody is using the PC.
pub const IDLE_LIMIT: Duration = Duration::from_secs(5 * 60);

/// Has the user walked away? Idle for 5 minutes, unless a video or game fills the screen.
pub fn away(idle: Duration, fullscreen: bool) -> bool {
    idle >= IDLE_LIMIT && !fullscreen
}

/// Seconds between samples, and the seconds each sample adds.
pub const SAMPLE_SECS: u64 = 5;

/// One day of screen time, as saved in its file. Program names only: never window titles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Day {
    pub date: NaiveDate,
    /// Seconds in front, per program.
    pub apps: BTreeMap<String, u64>,
    /// Seconds counted in each hour, 0 to 23.
    pub hours: [u64; 24],
    /// Seconds after midnight of the first and the last sample counted.
    pub first: Option<u32>,
    pub last: Option<u32>,
}

impl Day {
    pub fn new(date: NaiveDate) -> Self {
        Day { date, apps: BTreeMap::new(), hours: [0; 24], first: None, last: None }
    }
}

impl Day {
    /// Count one sample: `app` was in front at `at`.
    pub fn add(&mut self, app: &str, at: NaiveDateTime) {
        *self.apps.entry(app.to_string()).or_insert(0) += SAMPLE_SECS;
        self.hours[at.hour() as usize] += SAMPLE_SECS;
        let second = at.num_seconds_from_midnight();
        self.first.get_or_insert(second);
        self.last = Some(second);
    }
}

/// Kinetik's screen time folder: `%LocalAppData%\KinetikStats\ScreenTime`.
pub fn screen_time_dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    base.join("KinetikStats").join("ScreenTime")
}

/// Today's screen time, counted while Kinetik runs.
pub struct ScreenTime {
    pub day: Day,
    /// Where the day files go.
    pub dir: PathBuf,
    /// The 5-second sampling loop, once started.
    pub sampler: Option<Task<()>>,
}

impl ScreenTime {
    /// Start counting `today`, carrying on from its file if it has one.
    pub fn new(dir: PathBuf, today: NaiveDate) -> Self {
        let day = load_day(&dir, today).unwrap_or_else(|| Day::new(today));
        ScreenTime { day, dir, sampler: None }
    }

    /// Take a sample every 5 seconds until the counter is dropped.
    pub fn start(&mut self, desk: Box<dyn Desk>, cx: &mut Context<Self>) {
        self.sampler = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(SAMPLE_SECS)).await;
                let sampled = this.update(cx, |this, cx| {
                    this.sample(&*desk);
                    cx.notify();
                });
                if sampled.is_err() {
                    break;
                }
            }
        }));
    }
}

impl ScreenTime {
    /// One sample: a new day starts a new count, and the program in front, if it counts, gets 5 seconds, saved straight away.
    pub fn sample(&mut self, desk: &dyn Desk) {
        let now = desk.now();
        if now.date() != self.day.date {
            self.day = Day::new(now.date());
            let _ = prune(&self.dir, now.date());
        }
        if let Some(app) = app_to_count(desk) {
            self.day.add(&app, now);
            let _ = save_day(&self.dir, &self.day);
        }
    }
}

/// The file for one day: `2026-10-22.json` in `dir`.
pub fn day_path(dir: &Path, date: NaiveDate) -> PathBuf {
    dir.join(format!("{}.json", date.format("%Y-%m-%d")))
}

/// Write a day to its file, making the folder first if it is missing.
pub fn save_day(dir: &Path, day: &Day) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let text = serde_json::to_string_pretty(day)?;
    std::fs::write(day_path(dir, day.date), text)
}

/// A day's file, if it has one that reads.
pub fn load_day(dir: &Path, date: NaiveDate) -> Option<Day> {
    let text = std::fs::read_to_string(day_path(dir, date)).ok()?;
    serde_json::from_str(&text).ok()
}

/// How many days of screen time to keep.
pub const KEEP_DAYS: i64 = 400;

/// Delete the day files more than 400 days before `today`. Any other file is left alone. Gives back how many went.
pub fn prune(dir: &Path, today: NaiveDate) -> std::io::Result<usize> {
    let mut deleted = 0;
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let date = path.file_stem().and_then(|stem| stem.to_str()).and_then(|stem| NaiveDate::parse_from_str(stem, "%Y-%m-%d").ok());
        if path.extension().is_some_and(|ext| ext == "json")
            && let Some(date) = date
            && (today - date).num_days() > KEEP_DAYS
        {
            std::fs::remove_file(&path)?;
            deleted += 1;
        }
    }
    Ok(deleted)
}

/// What the screen time page says about a day.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DaySummary {
    /// Seconds counted, all programs together.
    pub total: u64,
    /// Seconds after midnight of the first and the last use.
    pub first: Option<u32>,
    pub last: Option<u32>,
    pub most_used: Option<String>,
    /// The hour, 0 to 23, with the most use.
    pub busiest_hour: Option<usize>,
}

/// Seconds as hours and minutes: 8100 gives "2 h 15 min".
pub fn hours_minutes(secs: u64) -> String {
    format!("{} h {:02} min", secs / 3600, secs / 60 % 60)
}

/// Seconds after midnight as a clock time: 75600 gives "21:00".
pub fn clock(second: u32) -> String {
    format!("{:02}:{:02}", second / 3600, second / 60 % 60)
}

/// Total, first and last use, the most used program and the busiest hour of a day.
pub fn summarize(day: &Day) -> DaySummary {
    DaySummary {
        total: day.apps.values().sum(),
        first: day.first,
        last: day.last,
        most_used: day.apps.iter().max_by_key(|(_, secs)| **secs).map(|(name, _)| name.clone()),
        busiest_hour: (0..24).filter(|&hour| day.hours[hour] > 0).max_by_key(|&hour| day.hours[hour]),
    }
}

/// Bar heights for the 24 hours: the busiest hour gets `tallest`, the others their share of it. An empty day is all zeros.
pub fn bar_heights(hours: &[u64; 24], tallest: f32) -> [f32; 24] {
    let busiest = hours.iter().copied().max().unwrap_or(0);
    if busiest == 0 {
        return [0.0; 24];
    }
    hours.map(|secs| tallest * secs as f32 / busiest as f32)
}
