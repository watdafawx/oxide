//! XP, levels, streaks, achievements. All game rules live here so the UI only renders.

use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDate, Timelike};
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
/// Chance of a critical clear (2x XP) on a first-time clear.
pub const CRIT_CHANCE: f64 = 0.12;
/// Each combo step adds this to the XP multiplier, up to `COMBO_CAP` steps.
pub const COMBO_STEP: f64 = 0.05;
pub const COMBO_CAP: u32 = 10;
/// A streak freeze is earned every this many streak days.
pub const FREEZE_EVERY: u32 = 7;
pub const MAX_FREEZES: u32 = 2;

/// XP multiplier for a combo of `combo` consecutive clears.
pub fn combo_multiplier(combo: u32) -> f64 {
    1.0 + COMBO_STEP * combo.min(COMBO_CAP) as f64
}

/// Roll in [0, 1) from a quest id and a timestamp. No RNG crate: a hash is plenty for a loot roll.
fn roll_for(quest_id: &str, now: DateTime<Local>) -> f64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    quest_id.hash(&mut h);
    now.timestamp_nanos_opt().unwrap_or_default().hash(&mut h);
    // top 53 bits -> exact f64 in [0, 1)
    (h.finish() >> 11) as f64 / (1u64 << 53) as f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub xp: u32,
    pub level: u32,
    pub xp_into_level: u32,
    pub xp_for_next: u32,
    pub streak_days: u32,
    pub best_streak: u32,
    pub last_active: Option<String>,
    /// Consecutive first-time clears without a hint or a reset.
    #[serde(default)]
    pub combo: u32,
    /// Streak freezes in stock (max `MAX_FREEZES`).
    #[serde(default)]
    pub freezes: u32,
    pub achievements: Vec<String>,
    pub completed: Vec<String>,
    pub started: Vec<String>,
    pub settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// External editor command (`code`, `zed`); empty = the in-app editor only.
    pub editor: String,
    pub sound: bool,
    pub focus_minutes: u32,
    /// Active track id; empty = the catalog's first track.
    #[serde(default)]
    pub track: String,
    /// How quests are told: `games` (default) or `plain`; falls back to plain per quest.
    #[serde(default = "default_flavour")]
    pub flavour: String,
    /// Master volume 0-1.
    #[serde(default = "default_volume")]
    pub volume: f32,
    #[serde(default)]
    pub sounds: SoundToggles,
}

/// Per-sound switches; `Settings::sound` stays the master switch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundToggles {
    pub objective: bool,
    pub clear: bool,
    pub level: bool,
    pub achievement: bool,
    pub crit: bool,
}

impl Default for SoundToggles {
    fn default() -> Self {
        Self { objective: true, clear: true, level: true, achievement: true, crit: true }
    }
}

fn default_volume() -> f32 {
    0.6
}

fn default_flavour() -> String {
    "games".into()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            editor: String::new(),
            sound: true,
            focus_minutes: 25,
            track: String::new(),
            flavour: default_flavour(),
            volume: default_volume(),
            sounds: SoundToggles::default(),
        }
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
    /// Critical clear: XP doubled.
    pub crit: bool,
    /// Combo after this clear.
    pub combo: u32,
    /// XP multiplier the combo gave this clear (1.0 = none).
    pub combo_mult: f64,
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

/// An achievement the engine can award. Hidden ones show as "???" until earned.
pub struct AchievementDef {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub hidden: bool,
}

const fn ach(id: &'static str, title: &'static str, description: &'static str) -> AchievementDef {
    AchievementDef { id, title, description, hidden: false }
}

const fn secret(id: &'static str, title: &'static str, description: &'static str) -> AchievementDef {
    AchievementDef { id, title, description, hidden: true }
}

/// Every achievement the engine can award, for the profile grid.
pub const ACHIEVEMENTS: &[AchievementDef] = &[
    ach("first-green", "First green", "Pass your first quest"),
    ach("no-hints", "Self-taught", "Clear a quest without a hint"),
    ach("speedrun", "Speedrun", "Clear a quest in under its estimate"),
    ach("streak-3", "Warming up", "3-day streak"),
    ach("streak-7", "On fire", "7-day streak"),
    ach("streak-30", "Unstoppable", "30-day streak"),
    ach("focus-first", "Deep work", "Clear a quest while the focus timer runs"),
    ach("boss", "Boss slayer", "Clear a boss"),
    ach("world", "World cleared", "Clear every quest in a world"),
    ach("ten-quests", "Grinder", "Clear 10 quests"),
    ach("clean-5", "Clippy's friend", "5 clean clears"),
    secret("early-bird", "Early bird", "Clear a quest before 07:00"),
    secret("night-owl", "Night owl", "Clear a quest after 23:00"),
    secret("one-save", "One save", "Go green on your first save"),
    secret("comeback", "Comeback", "Clear a quest after 7+ days away"),
    secret("crit-first", "Lucky strike", "Land your first critical clear"),
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
        // Columns added after the first release; the error is "duplicate column" on migrated files.
        for col in [
            "ALTER TABLE profile ADD COLUMN combo INTEGER NOT NULL DEFAULT 0",
            "ALTER TABLE profile ADD COLUMN freezes INTEGER NOT NULL DEFAULT 0",
            "ALTER TABLE profile ADD COLUMN prev_active TEXT",
        ] {
            let _ = conn.execute(col, []);
        }
        Ok(Self { conn })
    }

    pub fn profile(&self) -> Result<Profile> {
        type Row = (u32, u32, u32, Option<String>, String, u32, u32);
        let (xp, streak_days, best_streak, last_active, settings, combo, freezes): Row = self.conn.query_row(
            "SELECT xp, streak_days, best_streak, last_active, settings, combo, freezes FROM profile WHERE id = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?)),
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
            combo,
            freezes,
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
        // A hint on a quest still to clear breaks the combo.
        if self.run(quest_id)?.is_some_and(|r| r.completed_at.is_none()) {
            self.break_combo()?;
        }
        Ok(self.run(quest_id)?.map(|r| r.hints_used).unwrap_or(0))
    }

    /// A check ran and did not pass.
    pub fn record_attempt(&self, quest_id: &str, now: DateTime<Local>) -> Result<()> {
        self.conn.execute("UPDATE quest_runs SET attempts = attempts + 1 WHERE quest_id = ?1", params![quest_id])?;
        self.touch(now.date_naive())
    }

    /// Combo back to zero (hint used, quest reset).
    pub fn break_combo(&self) -> Result<()> {
        self.conn.execute("UPDATE profile SET combo = 0 WHERE id = 1", [])?;
        Ok(())
    }

    /// Streak bookkeeping for activity on `today`. Skipped days are covered by freezes when there are enough.
    pub fn touch(&self, today: NaiveDate) -> Result<()> {
        let (last, streak, freezes): (Option<String>, u32, u32) = self.conn.query_row(
            "SELECT last_active, streak_days, freezes FROM profile WHERE id = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )?;
        let last = last.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        let (new_streak, mut freezes) = match last {
            Some(d) if d == today => return Ok(()),
            Some(d) => {
                let missed = (today - d).num_days() - 1;
                if missed <= 0 {
                    (streak + 1, freezes)
                } else if missed <= freezes as i64 {
                    (streak + 1, freezes - missed as u32)
                } else {
                    (1, freezes)
                }
            }
            None => (1, freezes),
        };
        if new_streak > streak && new_streak % FREEZE_EVERY == 0 {
            freezes = (freezes + 1).min(MAX_FREEZES);
        }
        self.conn.execute(
            "UPDATE profile SET streak_days = ?1, best_streak = MAX(best_streak, ?1), freezes = ?3,
                    prev_active = last_active, last_active = ?2 WHERE id = 1",
            params![new_streak, today.format("%Y-%m-%d").to_string(), freezes],
        )?;
        Ok(())
    }

    /// `(streak, freezes)` as of `today`: the streak is 0 if more days were skipped than freezes cover,
    /// and freezes a pending skip would consume are already subtracted. Does not write.
    pub fn streak_and_freezes_on(&self, today: NaiveDate) -> Result<(u32, u32)> {
        let p = self.profile()?;
        let last = p.last_active.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
        Ok(match last {
            Some(d) => {
                let missed = ((today - d).num_days() - 1).max(0);
                if missed == 0 {
                    (p.streak_days, p.freezes)
                } else if missed <= p.freezes as i64 {
                    (p.streak_days, p.freezes - missed as u32)
                } else {
                    (0, p.freezes)
                }
            }
            None => (0, p.freezes),
        })
    }

    /// The streak as of `today`: 0 if the learner skipped more days than freezes cover. Does not write.
    pub fn streak_on(&self, today: NaiveDate) -> Result<u32> {
        Ok(self.streak_and_freezes_on(today)?.0)
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
        self.complete_quest_with(catalog, quest, now, focus_active, clean, roll_for(&quest.id, now))
    }

    /// `complete_quest` with the crit roll (in [0, 1)) injected; below `CRIT_CHANCE` is a crit.
    pub fn complete_quest_with(
        &self,
        catalog: &Catalog,
        quest: &Quest,
        now: DateTime<Local>,
        focus_active: bool,
        clean: bool,
        crit_roll: f64,
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
                crit: false,
                combo: before.combo,
                combo_mult: 1.0,
            });
        }

        let crit = crit_roll < CRIT_CHANCE;
        let combo_mult = combo_multiplier(before.combo);
        let combo = if run.hints_used > 0 { 0 } else { before.combo + 1 };
        let mut xp = quest.xp as f64 * combo_mult;
        if focus_active {
            xp *= FOCUS_MULTIPLIER;
        }
        if clean {
            xp *= 1.0 + CLEAN_BONUS;
        }
        if crit {
            xp *= 2.0;
        }
        let xp = xp.round() as u32;

        self.conn.execute(
            "UPDATE quest_runs SET completed_at = ?2, xp_awarded = ?3 WHERE quest_id = ?1",
            params![quest.id, now.to_rfc3339(), xp],
        )?;
        self.conn.execute("UPDATE profile SET xp = xp + ?1, combo = ?2 WHERE id = 1", params![xp, combo])?;

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
        // Hidden ones.
        if now.hour() < 7 {
            candidates.push("early-bird");
        }
        if now.hour() >= 23 {
            candidates.push("night-owl");
        }
        // attempts <= 1: the starter's own red run does not count against the learner.
        if run.attempts <= 1 && quest.has_project() {
            candidates.push("one-save");
        }
        let prev_active: Option<String> = self.conn.query_row("SELECT prev_active FROM profile WHERE id = 1", [], |r| r.get(0))?;
        let away = prev_active
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
            .map(|d| (now.date_naive() - d).num_days())
            .unwrap_or(0);
        if away >= 7 {
            candidates.push("comeback");
        }
        if crit {
            candidates.push("crit-first");
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
            crit,
            combo,
            combo_mult,
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
            flavour: "plain".into(),
            variants: vec![],
            track_id: "rust".into(),
            world_id: world.into(),
            index: n,
            slug: "q".into(),
            title: "Q".into(),
            summary: String::new(),
            kind,
            runner: Runner::Cargo,
            jdk: None,
            xp,
            minutes: 10,
            tags: vec![],
            requires: vec![],
            objectives: vec![Objective { test: "t".into(), label: "t".into() }],
            hints: vec![],
            quiz: vec![],
            points: vec![],
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
        let c = p.complete_quest_with(&cat, q, day(1), true, true, 0.9).unwrap();
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
        let c = p.complete_quest_with(&cat, boss, day(1), false, false, 0.9).unwrap();
        assert_eq!(c.xp_awarded, 315); // combo of 1 before this clear: x1.05
        assert!(c.level_after > c.level_before);
        assert!(c.new_achievements.contains(&"boss".to_string()));
        assert!(c.new_achievements.contains(&"world".to_string()));
        assert_eq!(p.profile().unwrap().xp, 398);
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
        let s = Settings {
            editor: "zed".into(),
            sound: false,
            focus_minutes: 15,
            track: "lua".into(),
            flavour: "plain".into(),
            volume: 0.3,
            sounds: SoundToggles { crit: false, ..Default::default() },
        };
        p.set_settings(&s).unwrap();
        let got = p.settings().unwrap();
        assert_eq!(got.editor, "zed");
        assert!(!got.sounds.crit && got.sounds.clear);
        assert!((got.volume - 0.3).abs() < 1e-6);
    }

    #[test]
    fn old_settings_json_gets_sound_defaults() {
        let s: Settings = serde_json::from_str(r#"{"editor":"","sound":true,"focus_minutes":25}"#).unwrap();
        assert!((s.volume - 0.6).abs() < 1e-6);
        assert!(s.sounds.objective && s.sounds.achievement);
    }

    #[test]
    fn crit_doubles_xp_with_injected_roll() {
        let cat = catalog();
        let q = &cat.worlds[0].quests[0];
        let p = Progress::in_memory().unwrap();
        let c = p.complete_quest_with(&cat, q, day(1), false, false, 0.05).unwrap();
        assert!(c.crit);
        assert_eq!(c.xp_awarded, 100);
        assert!(c.new_achievements.contains(&"crit-first".to_string()));

        let p = Progress::in_memory().unwrap();
        let c = p.complete_quest_with(&cat, q, day(1), false, false, 0.12).unwrap();
        assert!(!c.crit);
        assert_eq!(c.xp_awarded, 50);
        assert!(!c.new_achievements.contains(&"crit-first".to_string()));
        // repeat clears never crit
        let again = p.complete_quest_with(&cat, q, day(1), false, false, 0.0).unwrap();
        assert!(!again.crit && again.xp_awarded == 0);
    }

    #[test]
    fn roll_is_in_range_and_varies() {
        let a = roll_for("a/01-x", day(1));
        let b = roll_for("a/01-x", day(2));
        assert!((0.0..1.0).contains(&a) && (0.0..1.0).contains(&b));
        assert_ne!(a, b);
    }

    fn many_quests(n: u32) -> Catalog {
        Catalog {
            tracks: vec![],
            worlds: vec![World {
                id: "00-w".into(),
                track_id: "rust".into(),
                index: 0,
                slug: "w".into(),
                title: "W".into(),
                blurb: String::new(),
                quests: (1..=n).map(|i| quest("00-w", i, QuestKind::Exercise, 100)).collect(),
            }],
        }
    }

    #[test]
    fn combo_builds_multiplier_and_resets_on_hint_and_reset() {
        let cat = many_quests(4);
        let p = Progress::in_memory().unwrap();
        let qs = &cat.worlds[0].quests;
        let c1 = p.complete_quest_with(&cat, &qs[0], day(1), false, false, 0.9).unwrap();
        assert_eq!((c1.combo, c1.xp_awarded), (1, 100)); // multiplier from the combo *before* this clear
        let c2 = p.complete_quest_with(&cat, &qs[1], day(1), false, false, 0.9).unwrap();
        assert_eq!((c2.combo, c2.xp_awarded), (2, 105));
        // a hint on the next quest breaks it
        p.start_quest(&qs[2].id, day(1)).unwrap();
        p.use_hint(&qs[2].id, 3).unwrap();
        assert_eq!(p.profile().unwrap().combo, 0);
        let c3 = p.complete_quest_with(&cat, &qs[2], day(1), false, false, 0.9).unwrap();
        assert_eq!((c3.combo, c3.xp_awarded), (0, 100));
        p.complete_quest_with(&cat, &qs[3], day(1), false, false, 0.9).unwrap();
        assert_eq!(p.profile().unwrap().combo, 1);
        p.break_combo().unwrap();
        assert_eq!(p.profile().unwrap().combo, 0);
    }

    #[test]
    fn combo_multiplier_caps_at_ten() {
        assert_eq!(combo_multiplier(0), 1.0);
        assert!((combo_multiplier(4) - 1.2).abs() < 1e-9);
        assert!((combo_multiplier(10) - 1.5).abs() < 1e-9);
        assert!((combo_multiplier(99) - 1.5).abs() < 1e-9);
    }

    #[test]
    fn seventh_day_earns_a_freeze_capped_at_two() {
        let p = Progress::in_memory().unwrap();
        for d in 1..=6 {
            p.touch(day(d).date_naive()).unwrap();
        }
        assert_eq!(p.profile().unwrap().freezes, 0);
        p.touch(day(7).date_naive()).unwrap();
        assert_eq!(p.profile().unwrap().freezes, 1);
        for d in 8..=21 {
            p.touch(day(d).date_naive()).unwrap();
        }
        let pr = p.profile().unwrap();
        assert_eq!((pr.streak_days, pr.freezes), (21, 2)); // third freeze is capped
    }

    #[test]
    fn freeze_covers_a_skipped_day() {
        let p = Progress::in_memory().unwrap();
        for d in 1..=7 {
            p.touch(day(d).date_naive()).unwrap();
        }
        // day 8 skipped: streak is alive on the 9th, one freeze pending
        assert_eq!(p.streak_and_freezes_on(day(9).date_naive()).unwrap(), (7, 0));
        p.touch(day(9).date_naive()).unwrap();
        let pr = p.profile().unwrap();
        assert_eq!((pr.streak_days, pr.freezes), (8, 0));
        // two days skipped with no freeze left: resets
        assert_eq!(p.streak_on(day(12).date_naive()).unwrap(), 0);
        p.touch(day(12).date_naive()).unwrap();
        assert_eq!(p.profile().unwrap().streak_days, 1);
    }

    #[test]
    fn freeze_does_not_stretch_over_a_longer_gap() {
        let p = Progress::in_memory().unwrap();
        for d in 1..=7 {
            p.touch(day(d).date_naive()).unwrap();
        }
        // 2 days missed, 1 freeze: not enough
        assert_eq!(p.streak_on(day(10).date_naive()).unwrap(), 0);
        p.touch(day(10).date_naive()).unwrap();
        let pr = p.profile().unwrap();
        assert_eq!((pr.streak_days, pr.freezes), (1, 1));
    }

    fn at(d: u32, h: u32) -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 10, d, h, 0, 0).unwrap()
    }

    #[test]
    fn hidden_time_of_day_achievements() {
        let cat = many_quests(3);
        let qs = &cat.worlds[0].quests;
        let p = Progress::in_memory().unwrap();
        let c = p.complete_quest_with(&cat, &qs[0], at(1, 6), false, false, 0.9).unwrap();
        assert!(c.new_achievements.contains(&"early-bird".to_string()));
        assert!(!c.new_achievements.contains(&"night-owl".to_string()));
        let c = p.complete_quest_with(&cat, &qs[1], at(1, 23), false, false, 0.9).unwrap();
        assert!(c.new_achievements.contains(&"night-owl".to_string()));
        let c = p.complete_quest_with(&cat, &qs[2], at(1, 12), false, false, 0.9).unwrap();
        assert!(!c.new_achievements.iter().any(|a| a == "early-bird" || a == "night-owl"));
    }

    #[test]
    fn one_save_needs_no_failed_attempt_beyond_the_starter() {
        let cat = many_quests(2);
        let qs = &cat.worlds[0].quests;
        let p = Progress::in_memory().unwrap();
        p.start_quest(&qs[0].id, day(1)).unwrap();
        p.record_attempt(&qs[0].id, day(1)).unwrap(); // starter run
        assert!(p.complete_quest_with(&cat, &qs[0], day(1), false, false, 0.9).unwrap().new_achievements.contains(&"one-save".to_string()));
        p.start_quest(&qs[1].id, day(1)).unwrap();
        p.record_attempt(&qs[1].id, day(1)).unwrap();
        p.record_attempt(&qs[1].id, day(1)).unwrap();
        assert!(!p.complete_quest_with(&cat, &qs[1], day(1), false, false, 0.9).unwrap().new_achievements.contains(&"one-save".to_string()));
    }

    #[test]
    fn comeback_after_a_week_away() {
        let cat = many_quests(2);
        let qs = &cat.worlds[0].quests;
        let p = Progress::in_memory().unwrap();
        p.complete_quest_with(&cat, &qs[0], day(1), false, false, 0.9).unwrap();
        let c = p.complete_quest_with(&cat, &qs[1], day(9), false, false, 0.9).unwrap();
        assert!(c.new_achievements.contains(&"comeback".to_string()));

        let p = Progress::in_memory().unwrap();
        p.complete_quest_with(&cat, &qs[0], day(1), false, false, 0.9).unwrap();
        let c = p.complete_quest_with(&cat, &qs[1], day(3), false, false, 0.9).unwrap();
        assert!(!c.new_achievements.contains(&"comeback".to_string()));
    }

    #[test]
    fn hidden_flags_in_table() {
        let hidden: Vec<_> = ACHIEVEMENTS.iter().filter(|a| a.hidden).map(|a| a.id).collect();
        assert_eq!(hidden, ["early-bird", "night-owl", "one-save", "comeback", "crit-first"]);
    }
}
