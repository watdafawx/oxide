//! XP, levels, streaks, achievements. All game rules live here so the UI only renders.

use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDate};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

use crate::content::{Catalog, Quest, QuestKind};

/// XP needed to go from `level` to `level + 1`.
pub fn xp_to_next(level: u32) -> u32 {
    (100.0 * (level as f64).powf(1.5)).round() as u32
}

/// `(level, xp into this level, xp needed for next)` for a total.
pub fn level_for(total_xp: u32) -> (u32, u32, u32) {
    let mut level = 1;
    let mut rest = total_xp;
    loop {
        let need = xp_to_next(level);
        if rest < need {
            return (level, rest, need);
        }
        rest -= need;
        level += 1;
    }
}

/// Focus timer running while the quest went green.
pub const FOCUS_MULTIPLIER: f64 = 1.5;
/// Clippy had nothing to say.
pub const CLEAN_BONUS: f64 = 0.10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub xp: u32,
    pub level: u32,
    pub xp_into_level: u32,
    pub xp_for_next: u32,
    pub streak_days: u32,
    pub best_streak: u32,
    pub last_active: Option<String>,
    pub achievements: Vec<String>,
    pub completed: Vec<String>,
    pub started: Vec<String>,
    pub settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub editor: String,
    pub sound: bool,
    pub focus_minutes: u32,
    /// Active track id; empty = the catalog's first track.
    #[serde(default)]
    pub track: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self { editor: "code".into(), sound: true, focus_minutes: 25, track: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Completion {
    pub quest_id: String,
    pub xp_awarded: u32,
    pub level_before: u32,
    pub level_after: u32,
    pub new_achievements: Vec<String>,
    pub focus_bonus: bool,
    pub clean: bool,
    pub first_time: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRun {
    pub quest_id: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub hints_used: u32,
    pub attempts: u32,
    pub xp_awarded: u32,
}

/// Every achievement the engine can award, for the profile grid.
pub const ACHIEVEMENTS: &[(&str, &str, &str)] = &[
    ("first-green", "First green", "Pass your first quest"),
    ("no-hints", "Self-taught", "Clear a quest without a hint"),
    ("speedrun", "Speedrun", "Clear a quest in under its estimate"),
    ("streak-3", "Warming up", "3-day streak"),
    ("streak-7", "On fire", "7-day streak"),
    ("streak-30", "Unstoppable", "30-day streak"),
    ("focus-first", "Deep work", "Clear a quest while the focus timer runs"),
    ("boss", "Boss slayer", "Clear a boss"),
    ("world", "World cleared", "Clear every quest in a world"),
    ("ten-quests", "Grinder", "Clear 10 quests"),
    ("clean-5", "Clippy's friend", "5 clean clears"),
];

pub struct Progress {
    conn: Connection,
}

impl Progress {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p)?;
        }
        let conn = Connection::open(path).with_context(|| format!("opening {}", path.display()))?;
        Self::init(conn)
    }

    pub fn in_memory() -> Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<Self> {
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             CREATE TABLE IF NOT EXISTS profile (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                xp INTEGER NOT NULL DEFAULT 0,
                streak_days INTEGER NOT NULL DEFAULT 0,
                best_streak INTEGER NOT NULL DEFAULT 0,
                last_active TEXT,
                settings TEXT NOT NULL DEFAULT '{}'
             );
             INSERT OR IGNORE INTO profile (id) VALUES (1);
             CREATE TABLE IF NOT EXISTS quest_runs (
                quest_id TEXT PRIMARY KEY,
                started_at TEXT NOT NULL,
                completed_at TEXT,
                hints_used INTEGER NOT NULL DEFAULT 0,
                attempts INTEGER NOT NULL DEFAULT 0,
                xp_awarded INTEGER NOT NULL DEFAULT 0
             );
             CREATE TABLE IF NOT EXISTS achievements (
                id TEXT PRIMARY KEY,
                earned_at TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS focus_sessions (
                id INTEGER PRIMARY KEY,
                quest_id TEXT,
                started_at TEXT NOT NULL,
                minutes INTEGER NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
             );",
        )?;
        Ok(Self { conn })
    }

    pub fn profile(&self) -> Result<Profile> {
        let (xp, streak_days, best_streak, last_active, settings): (u32, u32, u32, Option<String>, String) =
            self.conn.query_row(
                "SELECT xp, streak_days, best_streak, last_active, settings FROM profile WHERE id = 1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )?;
        let (level, xp_into_level, xp_for_next) = level_for(xp);
        let achievements = self.column("SELECT id FROM achievements ORDER BY earned_at")?;
        let completed = self.column("SELECT quest_id FROM quest_runs WHERE completed_at IS NOT NULL ORDER BY completed_at")?;
        let started = self.column("SELECT quest_id FROM quest_runs WHERE completed_at IS NULL ORDER BY started_at")?;
        let settings = serde_json::from_str(&settings).unwrap_or_default();
        Ok(Profile {
            xp,
            level,
            xp_into_level,
            xp_for_next,
            streak_days,
            best_streak,
            last_active,
            achievements,
            completed,
            started,
            settings,
        })
    }

    pub fn completed_set(&self) -> Result<HashSet<String>> {
        Ok(self.column("SELECT quest_id FROM quest_runs WHERE completed_at IS NOT NULL")?.into_iter().collect())
    }

    pub fn settings(&self) -> Result<Settings> {
        Ok(self.profile()?.settings)
    }

    pub fn set_settings(&self, s: &Settings) -> Result<()> {
        self.conn.execute("UPDATE profile SET settings = ?1 WHERE id = 1", params![serde_json::to_string(s)?])?;
        Ok(())
    }

    pub fn run(&self, quest_id: &str) -> Result<Option<QuestRun>> {
        Ok(self
            .conn
            .query_row(
                "SELECT quest_id, started_at, completed_at, hints_used, attempts, xp_awarded FROM quest_runs WHERE quest_id = ?1",
                params![quest_id],
                |r| {
                    Ok(QuestRun {
                        quest_id: r.get(0)?,
                        started_at: r.get(1)?,
                        completed_at: r.get(2)?,
                        hints_used: r.get(3)?,
                        attempts: r.get(4)?,
                        xp_awarded: r.get(5)?,
                    })
                },
            )
            .optional()?)
    }

    /// Record that the learner opened the quest. Idempotent; counts for the streak.
    pub fn start_quest(&self, quest_id: &str, now: DateTime<Local>) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO quest_runs (quest_id, started_at) VALUES (?1, ?2)",
            params![quest_id, now.to_rfc3339()],
        )?;
        self.touch(now.date_naive())?;
        Ok(())
    }

    /// Hints revealed so far, after revealing one more (capped at `total`).
    pub fn use_hint(&self, quest_id: &str, total: u32) -> Result<u32> {
        self.conn.execute(
            "UPDATE quest_runs SET hints_used = MIN(hints_used + 1, ?2) WHERE quest_id = ?1",
            params![quest_id, total],
        )?;
        Ok(self.run(quest_id)?.map(|r| r.hints_used).unwrap_or(0))
    }

    /// A check ran and did not pass.
    pub fn record_attempt(&self, quest_id: &str, now: DateTime<Local>) -> Result<()> {
        self.conn.execute("UPDATE quest_runs SET attempts = attempts + 1 WHERE quest_id = ?1", params![quest_id])?;
        self.touch(now.date_naive())
    }

    /// Streak bookkeeping for activity on `today`.
    pub fn touch(&self, today: NaiveDate) -> Result<()> {
        let last: Option<String> = self.conn.query_row("SELECT last_active FROM profile WHERE id = 1", [], |r| r.get(0))?;
        let last = last.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        let streak: u32 = self.conn.query_row("SELECT streak_days FROM profile WHERE id = 1", [], |r| r.get(0))?;
        let new_streak = match last {
            Some(d) if d == today => return Ok(()),
            Some(d) if d.succ_opt() == Some(today) => streak + 1,
            _ => 1,
        };
        self.conn.execute(
            "UPDATE profile SET streak_days = ?1, best_streak = MAX(best_streak, ?1), last_active = ?2 WHERE id = 1",
            params![new_streak, today.format("%Y-%m-%d").to_string()],
        )?;
        Ok(())
    }

    /// The streak as of `today`: 0 if the learner skipped a day. Call from the UI; does not write.
    pub fn streak_on(&self, today: NaiveDate) -> Result<u32> {
        let p = self.profile()?;
        let last = p.last_active.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        Ok(match last {
            Some(d) if d == today || d.succ_opt() == Some(today) => p.streak_days,
            _ => 0,
        })
    }

    /// The quest went green. Awards XP once; later clears return `first_time = false` and 0 XP.
    pub fn complete_quest(
        &self,
        catalog: &Catalog,
        quest: &Quest,
        now: DateTime<Local>,
        focus_active: bool,
        clean: bool,
    ) -> Result<Completion> {
        self.start_quest(&quest.id, now)?;
        let before = self.profile()?;
        let run = self.run(&quest.id)?.expect("run exists after start");
        if run.completed_at.is_some() {
            return Ok(Completion {
                quest_id: quest.id.clone(),
                xp_awarded: 0,
                level_before: before.level,
                level_after: before.level,
                new_achievements: vec![],
                focus_bonus: false,
                clean,
                first_time: false,
            });
        }

        let mut xp = quest.xp as f64;
        if focus_active {
            xp *= FOCUS_MULTIPLIER;
        }
        if clean {
            xp *= 1.0 + CLEAN_BONUS;
        }
        let xp = xp.round() as u32;

        self.conn.execute(
            "UPDATE quest_runs SET completed_at = ?2, xp_awarded = ?3 WHERE quest_id = ?1",
            params![quest.id, now.to_rfc3339(), xp],
        )?;
        self.conn.execute("UPDATE profile SET xp = xp + ?1 WHERE id = 1", params![xp])?;

        // Achievements.
        let started = DateTime::parse_from_rfc3339(&run.started_at).map(|d| d.with_timezone(&Local)).ok();
        let elapsed_min = started.map(|s| (now - s).num_minutes()).unwrap_or(i64::MAX);
        let after = self.profile()?;
        let completed: HashSet<String> = after.completed.iter().cloned().collect();
        let world_done = catalog
            .world(&quest.world_id)
            .is_some_and(|w| w.quests.iter().all(|q| completed.contains(&q.id)));
        let clean_count: u32 = self
            .conn
            .query_row("SELECT COUNT(*) FROM quest_runs WHERE completed_at IS NOT NULL", [], |r| r.get(0))?;

        let mut candidates: Vec<&str> = vec!["first-green"];
        if run.hints_used == 0 && quest.has_project() {
            candidates.push("no-hints");
        }
        if elapsed_min >= 0 && elapsed_min < quest.minutes as i64 && quest.has_project() {
            candidates.push("speedrun");
        }
        if focus_active {
            candidates.push("focus-first");
        }
        if quest.kind == QuestKind::Boss {
            candidates.push("boss");
        }
        if world_done {
            candidates.push("world");
        }
        if after.completed.len() >= 10 {
            candidates.push("ten-quests");
        }
        if clean && clean_count >= 5 {
            candidates.push("clean-5");
        }
        for (id, days) in [("streak-3", 3), ("streak-7", 7), ("streak-30", 30)] {
            if after.streak_days >= days {
                candidates.push(id);
            }
        }
        let mut new_achievements = Vec::new();
        for id in candidates {
            if self.award(id, now)? {
                new_achievements.push(id.to_string());
            }
        }

        Ok(Completion {
            quest_id: quest.id.clone(),
            xp_awarded: xp,
            level_before: before.level,
            level_after: after.level,
            new_achievements,
            focus_bonus: focus_active,
            clean,
            first_time: true,
        })
    }

    /// True if newly earned.
    fn award(&self, id: &str, now: DateTime<Local>) -> Result<bool> {
        let n = self.conn.execute(
            "INSERT OR IGNORE INTO achievements (id, earned_at) VALUES (?1, ?2)",
            params![id, now.to_rfc3339()],
        )?;
        Ok(n == 1)
    }

    pub fn start_focus(&self, quest_id: Option<&str>, minutes: u32, now: DateTime<Local>) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO focus_sessions (quest_id, started_at, minutes) VALUES (?1, ?2, ?3)",
            params![quest_id, now.to_rfc3339(), minutes],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn finish_focus(&self, id: i64, completed: bool) -> Result<()> {
        self.conn.execute("UPDATE focus_sessions SET completed = ?2 WHERE id = ?1", params![id, completed as i32])?;
        Ok(())
    }

    /// Minutes of focus sessions, completed or not, for the profile page.
    pub fn focus_minutes_total(&self) -> Result<u32> {
        Ok(self.conn.query_row("SELECT COALESCE(SUM(minutes), 0) FROM focus_sessions WHERE completed = 1", [], |r| r.get(0))?)
    }

    /// Dates with any activity, for a streak calendar.
    pub fn active_dates(&self) -> Result<Vec<String>> {
        let mut dates: Vec<String> = self.column(
            "SELECT DISTINCT substr(ts, 1, 10) FROM (
                SELECT started_at AS ts FROM quest_runs
                UNION ALL SELECT completed_at FROM quest_runs WHERE completed_at IS NOT NULL
                UNION ALL SELECT started_at FROM focus_sessions
             ) ORDER BY 1",
        )?;
        dates.dedup();
        Ok(dates)
    }

    fn column(&self, sql: &str) -> Result<Vec<String>> {
        let mut st = self.conn.prepare(sql)?;
        let rows = st.query_map([], |r| r.get::<_, String>(0))?;
        Ok(rows.collect::<Result<_, _>>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{Objective, Runner, World};
    use chrono::TimeZone;
    use std::path::PathBuf;

    fn quest(world: &str, n: u32, kind: QuestKind, xp: u32) -> Quest {
        Quest {
            id: format!("{world}/{n:02}-q"),
            track_id: "rust".into(),
            world_id: world.into(),
            index: n,
            slug: "q".into(),
            title: "Q".into(),
            summary: String::new(),
            kind,
            runner: Runner::Cargo,
            xp,
            minutes: 10,
            tags: vec![],
            requires: vec![],
            objectives: vec![Objective { test: "t".into(), label: "t".into() }],
            hints: vec![],
            quiz: vec![],
            starter_fails_to_compile: false,
            dir: PathBuf::new(),
            lib_dir: None,
        }
    }

    fn catalog() -> Catalog {
        Catalog {
            tracks: vec![],
            worlds: vec![World {
                id: "00-w".into(),
                track_id: "rust".into(),
                index: 0,
                slug: "w".into(),
                title: "W".into(),
                blurb: String::new(),
                quests: vec![quest("00-w", 1, QuestKind::Exercise, 50), quest("00-w", 2, QuestKind::Boss, 300)],
            }],
        }
    }

    fn day(d: u32) -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 10, d, 12, 0, 0).unwrap()
    }

    #[test]
    fn level_curve() {
        assert_eq!(xp_to_next(1), 100);
        assert_eq!(level_for(0), (1, 0, 100));
        assert_eq!(level_for(100), (2, 0, 283));
        assert_eq!(level_for(150), (2, 50, 283));
    }

    #[test]
    fn complete_awards_once_and_levels_up() {
        let p = Progress::in_memory().unwrap();
        let cat = catalog();
        let q = &cat.worlds[0].quests[0];
        p.start_quest(&q.id, day(1)).unwrap();
        let c = p.complete_quest(&cat, q, day(1), true, true).unwrap();
        assert_eq!(c.xp_awarded, 83); // 50 * 1.5 * 1.1 = 82.5
        assert!(c.first_time);
        assert!(c.new_achievements.contains(&"first-green".to_string()));
        assert!(c.new_achievements.contains(&"no-hints".to_string()));
        assert!(c.new_achievements.contains(&"focus-first".to_string()));
        assert!(c.new_achievements.contains(&"speedrun".to_string()));
        let again = p.complete_quest(&cat, q, day(1), false, false).unwrap();
        assert_eq!(again.xp_awarded, 0);
        assert!(!again.first_time);

        let boss = &cat.worlds[0].quests[1];
        let c = p.complete_quest(&cat, boss, day(1), false, false).unwrap();
        assert_eq!(c.xp_awarded, 300);
        assert!(c.level_after > c.level_before);
        assert!(c.new_achievements.contains(&"boss".to_string()));
        assert!(c.new_achievements.contains(&"world".to_string()));
        assert_eq!(p.profile().unwrap().xp, 383);
    }

    #[test]
    fn streak_counts_consecutive_days_only() {
        let p = Progress::in_memory().unwrap();
        p.touch(day(1).date_naive()).unwrap();
        p.touch(day(1).date_naive()).unwrap();
        p.touch(day(2).date_naive()).unwrap();
        p.touch(day(3).date_naive()).unwrap();
        assert_eq!(p.profile().unwrap().streak_days, 3);
        assert_eq!(p.streak_on(day(4).date_naive()).unwrap(), 3);
        assert_eq!(p.streak_on(day(5).date_naive()).unwrap(), 0);
        p.touch(day(6).date_naive()).unwrap();
        let pr = p.profile().unwrap();
        assert_eq!(pr.streak_days, 1);
        assert_eq!(pr.best_streak, 3);
    }

    #[test]
    fn hints_cap_and_settings_roundtrip() {
        let p = Progress::in_memory().unwrap();
        p.start_quest("a/01-x", day(1)).unwrap();
        assert_eq!(p.use_hint("a/01-x", 2).unwrap(), 1);
        assert_eq!(p.use_hint("a/01-x", 2).unwrap(), 2);
        assert_eq!(p.use_hint("a/01-x", 2).unwrap(), 2);
        let s = Settings { editor: "zed".into(), sound: false, focus_minutes: 15, track: "lua".into() };
        p.set_settings(&s).unwrap();
        assert_eq!(p.settings().unwrap().editor, "zed");
    }
}
