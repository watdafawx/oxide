use app_boss_oxide_in_rust::*;
use std::cell::Cell;
use std::panic;

/// A pretend engine: every quest has two objectives, and the first check passes only one.
struct FakeApi {
    xp: Cell<u32>,
    checks: Cell<u32>,
}

fn quest() -> Quest {
    Quest {
        id: "demo".into(),
        title: "Hello, egui".into(),
        summary: "Show a heading and a button.".into(),
        xp: 50,
        minutes: 10,
        objectives: vec![
            Objective { test: "heading".into(), label: "Shows a heading".into() },
            Objective { test: "button".into(), label: "Shows a button".into() },
        ],
    }
}

impl FakeApi {
    fn profile(&self) -> Profile {
        let xp = self.xp.get();
        Profile { xp, level: xp / 100 + 1, xp_into_level: xp % 100, xp_for_next: 100, streak_days: 3 }
    }
}

impl Api for FakeApi {
    fn home(&self) -> Home {
        Home { next: Some(quest()), profile: self.profile() }
    }
    fn start_quest(&self, _id: &str) -> QuestDetail {
        QuestDetail { quest: quest(), lesson: String::new(), completed: false, last_check: None }
    }
    fn run_checks(&self, _id: &str) -> CheckOutcome {
        let n = self.checks.get();
        self.checks.set(n + 1);
        let all = n > 0;
        let objectives = quest()
            .objectives
            .into_iter()
            .enumerate()
            .map(|(i, o)| ObjectiveResult {
                test: o.test,
                label: o.label,
                status: if all || i == 0 { ObjectiveStatus::Passed } else { ObjectiveStatus::Failed },
                output: None,
            })
            .collect();
        let before = self.profile().level;
        let completion = all.then(|| {
            self.xp.set(self.xp.get() + 50);
            Completion { quest_id: "demo".into(), xp_awarded: 50, level_before: before, level_after: self.profile().level }
        });
        CheckOutcome { result: CheckResult { ok: all, compiled: true, objectives, duration_ms: 120 }, completion }
    }
}

fn show(app: &OxideApp<FakeApi>) {
    println!(
        "  screen {:?}, xp bar {:.0}%, objectives passed {}, toast {:?}, level-up {}",
        app.screen,
        xp_fraction(&app.home.profile) * 100.0,
        app.last.as_ref().map_or(0, passed_count),
        app.toast,
        app.level_up.is_some(),
    );
}

fn main() {
    println!("== Oxide in Rust ==");
    println!("Driving the app state with a fake engine (no window; the egui drawing is for the tests).");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let mut app = OxideApp::new(FakeApi { xp: Cell::new(80), checks: Cell::new(0) });
        println!("Home:");
        show(&app);
        println!("Start the quest:");
        app.start("demo");
        show(&app);
        println!("Run checks (one objective fails):");
        app.run_checks();
        show(&app);
        println!("Run checks again (all pass, level up):");
        app.run_checks();
        show(&app);
        println!("Back to Home:");
        app.go_home();
        show(&app);
    });
    if result.is_err() {
        println!("Finish `xp_fraction`, `passed_count`, `start`, `run_checks` and `go_home` to drive the app.");
    }
}
