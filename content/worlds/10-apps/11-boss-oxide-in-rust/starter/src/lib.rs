//! Oxide's Home and Quest screens in egui. The UI only talks to the `Api` trait.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Data shapes (a trimmed copy of the Oxide API contract). Done for you.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Objective {
    pub test: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub xp: u32,
    pub minutes: u32,
    pub objectives: Vec<Objective>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub xp: u32,
    pub level: u32,
    pub xp_into_level: u32,
    pub xp_for_next: u32,
    pub streak_days: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Home {
    pub next: Option<Quest>,
    pub profile: Profile,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectiveStatus {
    Passed,
    Failed,
    NotRun,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectiveResult {
    pub test: String,
    pub label: String,
    pub status: ObjectiveStatus,
    pub output: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckResult {
    pub ok: bool,
    pub compiled: bool,
    pub objectives: Vec<ObjectiveResult>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Completion {
    pub quest_id: String,
    pub xp_awarded: u32,
    pub level_before: u32,
    pub level_after: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestDetail {
    pub quest: Quest,
    pub lesson: String,
    pub completed: bool,
    pub last_check: Option<CheckResult>,
}

/// What the engine reports after a check run (the `check:result` event).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckOutcome {
    pub result: CheckResult,
    pub completion: Option<Completion>,
}

/// Everything the UI needs from the engine.
pub trait Api {
    fn home(&self) -> Home;
    fn start_quest(&self, id: &str) -> QuestDetail;
    fn run_checks(&self, id: &str) -> CheckOutcome;
}

// ---------------------------------------------------------------------------
// Pure helpers
// ---------------------------------------------------------------------------

/// How full the XP bar is: `xp_into_level / xp_for_next`, always between 0.0 and 1.0.
/// An `xp_for_next` of 0 gives 0.0.
pub fn xp_fraction(profile: &Profile) -> f32 {
    // TODO: return 0.0 when xp_for_next == 0, else (xp_into_level as f32 / xp_for_next as f32).clamp(0.0, 1.0)
    todo!()
}

/// How many objectives in the result have the status `Passed`.
pub fn passed_count(result: &CheckResult) -> usize {
    // TODO: result.objectives.iter().filter(|o| o.status == ObjectiveStatus::Passed).count()
    todo!()
}

// ---------------------------------------------------------------------------
// The app
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Home,
    Quest,
}

pub struct OxideApp<A: Api> {
    api: A,
    pub screen: Screen,
    pub home: Home,
    /// Set while the quest screen is open.
    pub detail: Option<QuestDetail>,
    /// The latest check result shown on the quest screen.
    pub last: Option<CheckResult>,
    /// For example `+50 XP`, shown after a first-time completion.
    pub toast: Option<String>,
    /// Set when a Completion raised the level; shown in a modal until dismissed.
    pub level_up: Option<Completion>,
}

impl<A: Api> OxideApp<A> {
    pub fn new(api: A) -> Self {
        let home = api.home();
        OxideApp {
            api,
            screen: Screen::Home,
            home,
            detail: None,
            last: None,
            toast: None,
            level_up: None,
        }
    }

    pub fn api(&self) -> &A {
        &self.api
    }

    /// Draws the current screen, then the level-up modal when one is pending.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match self.screen {
            Screen::Home => self.home_ui(ui),
            Screen::Quest => self.quest_ui(ui),
        }
        if self.level_up.is_some() {
            let ctx = ui.ctx().clone();
            self.level_up_modal(&ctx);
        }
    }

    // -- state changes ------------------------------------------------------

    /// Opens the quest: `detail` from `api.start_quest(id)`, `last` from its `last_check`,
    /// no toast, and `screen = Screen::Quest`.
    pub fn start(&mut self, id: &str) {
        // TODO: let detail = self.api.start_quest(id); self.last = detail.last_check.clone(); self.detail = Some(detail); self.toast = None; self.screen = Screen::Quest
        todo!()
    }

    /// Runs the checks for the open quest and applies the outcome: store `result` in `last`;
    /// with a completion, set `toast` to `+{xp} XP` when `xp_awarded > 0`, and set
    /// `level_up` when `level_after > level_before`.
    pub fn run_checks(&mut self) {
        // TODO: quest id from self.detail (return if None); outcome = self.api.run_checks(&id); self.last = Some(outcome.result); handle outcome.completion (toast + level_up)
        todo!()
    }

    /// Back to Home, reloading it from the Api and clearing the quest state.
    pub fn go_home(&mut self) {
        // TODO: self.home = self.api.home(); screen = Home; detail, last and toast = None
        todo!()
    }

    // -- drawing ------------------------------------------------------------

    /// Heading `Oxide`, labels `Level {level}` and `{xp_into_level} / {xp_for_next} XP`,
    /// a ProgressBar from `xp_fraction`, then the next-quest card (heading with the title,
    /// summary, `{minutes} min, {xp} XP`, and a `Start` button). With no next quest, a
    /// label `All caught up`.
    fn home_ui(&mut self, ui: &mut egui::Ui) {
        // TODO: draw the header + XP bar + card; remember the clicked quest id in a local Option<String> and call self.start(&id) after the match
        todo!()
    }

    /// Heading with the quest title; a label `{passed}/{total} objectives`; one label per
    /// objective, `[x] {label}` when its test passed in `last`, else `[ ] {label}`; the
    /// toast if any; buttons `Run checks` and `Back`. `Back` or the Esc key (while no
    /// level-up modal is open) calls `go_home`.
    fn quest_ui(&mut self, ui: &mut egui::Ui) {
        // TODO: clone title + objectives out of self.detail, draw heading/count/objective rows/toast, then the two buttons; act on clicks and Esc at the end
        todo!()
    }

    /// A modal with the heading `Level up!`, the label `You reached level {level_after}`
    /// and a `Continue` button. Clears `level_up` when Continue is clicked or the modal
    /// asks to close (`response.should_close()`).
    fn level_up_modal(&mut self, ctx: &egui::Context) {
        // TODO: let Some(c) = self.level_up.clone(), show egui::Modal::new(egui::Id::unique("level_up")).show(ctx, ..), clear self.level_up on Continue or should_close()
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn quest() -> Quest {
        Quest {
            id: "01-foundations/01-variables".into(),
            title: "Variables".into(),
            summary: "Bind a value to a name.".into(),
            xp: 40,
            minutes: 5,
            objectives: vec![
                Objective {
                    test: "binds_a_number".into(),
                    label: "Bind a number".into(),
                },
                Objective {
                    test: "shadows_it".into(),
                    label: "Shadow it".into(),
                },
            ],
        }
    }

    fn profile(level: u32, into: u32) -> Profile {
        Profile {
            xp: 140,
            level,
            xp_into_level: into,
            xp_for_next: 100,
            streak_days: 2,
        }
    }

    fn result(first: ObjectiveStatus, second: ObjectiveStatus) -> CheckResult {
        let q = quest();
        let statuses = [first, second];
        CheckResult {
            ok: statuses.iter().all(|s| *s == ObjectiveStatus::Passed),
            compiled: true,
            objectives: q
                .objectives
                .iter()
                .zip(statuses)
                .map(|(o, status)| ObjectiveResult {
                    test: o.test.clone(),
                    label: o.label.clone(),
                    status,
                    output: None,
                })
                .collect(),
            duration_ms: 12,
        }
    }

    fn completion(level_before: u32, level_after: u32) -> Completion {
        Completion {
            quest_id: quest().id,
            xp_awarded: 50,
            level_before,
            level_after,
        }
    }

    /// Scripted Api: answers with canned data and records every call.
    struct FakeApi {
        outcomes: RefCell<VecDeque<CheckOutcome>>,
        calls: RefCell<Vec<String>>,
    }

    impl FakeApi {
        fn new(outcomes: Vec<CheckOutcome>) -> Self {
            FakeApi {
                outcomes: RefCell::new(outcomes.into()),
                calls: RefCell::new(vec![]),
            }
        }
    }

    impl Api for FakeApi {
        fn home(&self) -> Home {
            self.calls.borrow_mut().push("home".into());
            Home {
                next: Some(quest()),
                profile: profile(1, 40),
            }
        }

        fn start_quest(&self, id: &str) -> QuestDetail {
            self.calls.borrow_mut().push(format!("start:{id}"));
            QuestDetail {
                quest: quest(),
                lesson: "# Variables".into(),
                completed: false,
                last_check: None,
            }
        }

        fn run_checks(&self, id: &str) -> CheckOutcome {
            self.calls.borrow_mut().push(format!("check:{id}"));
            self.outcomes
                .borrow_mut()
                .pop_front()
                .expect("the test did not script another check result")
        }
    }

    fn harness(outcomes: Vec<CheckOutcome>) -> Harness<'static, OxideApp<FakeApi>> {
        Harness::builder().with_max_steps(30).build_ui_state(
            |ui, app: &mut OxideApp<FakeApi>| app.ui(ui),
            OxideApp::new(FakeApi::new(outcomes)),
        )
    }

    fn red() -> CheckOutcome {
        CheckOutcome {
            result: result(ObjectiveStatus::Failed, ObjectiveStatus::NotRun),
            completion: None,
        }
    }

    fn green(completion: Completion) -> CheckOutcome {
        CheckOutcome {
            result: result(ObjectiveStatus::Passed, ObjectiveStatus::Passed),
            completion: Some(completion),
        }
    }

    #[test]
    fn xp_fraction_is_progress_through_level() {
        assert_eq!(xp_fraction(&profile(1, 25)), 0.25);
        assert_eq!(xp_fraction(&profile(1, 0)), 0.0);
        assert_eq!(xp_fraction(&profile(1, 100)), 1.0);
        let mut p = profile(1, 10);
        p.xp_for_next = 0;
        assert_eq!(xp_fraction(&p), 0.0, "no division by zero");
        assert_eq!(xp_fraction(&profile(1, 250)), 1.0, "never above 1");
    }

    #[test]
    fn home_shows_next_quest_card() {
        let harness = harness(vec![]);
        harness.get_by_label("Oxide");
        harness.get_by_label("Level 1");
        harness.get_by_label("40 / 100 XP");
        harness.get_by_label("Variables");
        harness.get_by_label("Bind a value to a name.");
        harness.get_by_label("5 min, 40 XP");
        harness.get_by_label("Start");
    }

    #[test]
    fn start_opens_the_quest_screen() {
        let mut harness = harness(vec![]);
        harness.get_by_label("Start").click();
        harness.run();
        harness.get_by_label("Variables");
        harness.get_by_label("0/2 objectives");
        harness.get_by_label("[ ] Bind a number");
        harness.get_by_label("[ ] Shadow it");
        assert!(
            harness
                .state()
                .api()
                .calls
                .borrow()
                .contains(&"start:01-foundations/01-variables".to_string()),
            "Start must call Api::start_quest with the quest id"
        );
    }

    #[test]
    fn red_check_shows_zero_of_n() {
        let mut harness = harness(vec![red()]);
        harness.get_by_label("Start").click();
        harness.run();
        harness.get_by_label("Run checks").click();
        harness.run();
        harness.get_by_label("0/2 objectives");
        harness.get_by_label("[ ] Bind a number");
        assert!(harness.query_by_label("+50 XP").is_none(), "no toast on red");
        assert!(harness.query_by_label("Level up!").is_none());
    }

    #[test]
    fn green_check_ticks_all_and_shows_xp_toast() {
        let mut harness = harness(vec![green(completion(1, 1))]);
        harness.get_by_label("Start").click();
        harness.run();
        harness.get_by_label("Run checks").click();
        harness.run();
        harness.get_by_label("2/2 objectives");
        harness.get_by_label("[x] Bind a number");
        harness.get_by_label("[x] Shadow it");
        harness.get_by_label("+50 XP");
        assert!(
            harness.query_by_label("Level up!").is_none(),
            "same level: no modal"
        );
    }

    #[test]
    fn level_up_shows_modal_until_continue() {
        let mut harness = harness(vec![green(completion(1, 2))]);
        harness.get_by_label("Start").click();
        harness.run();
        harness.get_by_label("Run checks").click();
        harness.run();
        harness.get_by_label("Level up!");
        harness.get_by_label("You reached level 2");
        harness.get_by_label("Continue").click();
        harness.run();
        assert!(harness.query_by_label("Level up!").is_none(), "Continue closes it");
        harness.get_by_label("+50 XP");
    }

    #[test]
    fn escape_returns_home() {
        let mut harness = harness(vec![]);
        harness.get_by_label("Start").click();
        harness.run();
        harness.get_by_label("Run checks");
        harness.key_press(egui::Key::Escape);
        harness.run();
        harness.get_by_label("Start");
        assert_eq!(harness.state().screen, Screen::Home);
        let homes = harness
            .state()
            .api()
            .calls
            .borrow()
            .iter()
            .filter(|c| *c == "home")
            .count();
        assert_eq!(homes, 2, "going home reloads Home from the Api");
    }
}
