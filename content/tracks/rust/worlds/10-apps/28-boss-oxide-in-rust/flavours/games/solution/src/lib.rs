//! The mod manager's Home and Pack screens in egui. The UI only talks to the `Api` trait.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Data shapes (a trimmed copy of the mod manager's API contract). Done for you.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackMod {
    pub file: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Modpack {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub size_mb: u32,
    pub loader: String,
    pub mods: Vec<PackMod>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibraryStats {
    pub mb: u32,
    pub tier: u32,
    pub mb_into_tier: u32,
    pub mb_for_next: u32,
    pub packs_installed: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Home {
    pub next: Option<Modpack>,
    pub stats: LibraryStats,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileStatus {
    Verified,
    Broken,
    NotChecked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileResult {
    pub file: String,
    pub name: String,
    pub status: FileStatus,
    pub output: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyResult {
    pub ok: bool,
    pub downloaded: bool,
    pub files: Vec<FileResult>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Install {
    pub pack_id: String,
    pub mb_added: u32,
    pub tier_before: u32,
    pub tier_after: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackDetail {
    pub pack: Modpack,
    pub notes: String,
    pub installed: bool,
    pub last_verify: Option<VerifyResult>,
}

/// What the manager reports after verifying a pack's files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyOutcome {
    pub result: VerifyResult,
    pub install: Option<Install>,
}

/// Everything the UI needs from the mod manager's engine.
pub trait Api {
    fn home(&self) -> Home;
    fn open_pack(&self, id: &str) -> PackDetail;
    fn verify(&self, id: &str) -> VerifyOutcome;
}

// ---------------------------------------------------------------------------
// Pure helpers
// ---------------------------------------------------------------------------

/// How full the tier bar is: `mb_into_tier / mb_for_next`, always between 0.0 and 1.0.
/// An `mb_for_next` of 0 gives 0.0.
pub fn mb_fraction(stats: &LibraryStats) -> f32 {
    if stats.mb_for_next == 0 {
        return 0.0;
    }
    (stats.mb_into_tier as f32 / stats.mb_for_next as f32).clamp(0.0, 1.0)
}

/// How many files in the result have the status `Verified`.
pub fn verified_count(result: &VerifyResult) -> usize {
    result
        .files
        .iter()
        .filter(|f| f.status == FileStatus::Verified)
        .count()
}

// ---------------------------------------------------------------------------
// The app
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Home,
    Pack,
}

pub struct ModManagerApp<A: Api> {
    api: A,
    pub screen: Screen,
    pub home: Home,
    /// Set while the pack screen is open.
    pub detail: Option<PackDetail>,
    /// The latest verify result shown on the pack screen.
    pub last: Option<VerifyResult>,
    /// For example `+50 MB`, shown after a first-time install.
    pub toast: Option<String>,
    /// Set when an Install raised the tier; shown in a modal until dismissed.
    pub tier_up: Option<Install>,
}

impl<A: Api> ModManagerApp<A> {
    pub fn new(api: A) -> Self {
        let home = api.home();
        ModManagerApp {
            api,
            screen: Screen::Home,
            home,
            detail: None,
            last: None,
            toast: None,
            tier_up: None,
        }
    }

    pub fn api(&self) -> &A {
        &self.api
    }

    /// Draws the current screen, then the tier-up modal when one is pending.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match self.screen {
            Screen::Home => self.home_ui(ui),
            Screen::Pack => self.pack_ui(ui),
        }
        if self.tier_up.is_some() {
            let ctx = ui.ctx().clone();
            self.tier_up_modal(&ctx);
        }
    }

    // -- state changes ------------------------------------------------------

    /// Opens the pack: `detail` from `api.open_pack(id)`, `last` from its `last_verify`,
    /// no toast, and `screen = Screen::Pack`.
    pub fn open(&mut self, id: &str) {
        let detail = self.api.open_pack(id);
        self.last = detail.last_verify.clone();
        self.detail = Some(detail);
        self.toast = None;
        self.screen = Screen::Pack;
    }

    /// Verifies the open pack and applies the outcome: store `result` in `last`;
    /// with an install, set `toast` to `+{mb} MB` when `mb_added > 0`, and set
    /// `tier_up` when `tier_after > tier_before`.
    pub fn verify(&mut self) {
        let id = match &self.detail {
            Some(detail) => detail.pack.id.clone(),
            None => return,
        };
        let outcome = self.api.verify(&id);
        self.last = Some(outcome.result);
        if let Some(install) = outcome.install {
            if install.mb_added > 0 {
                self.toast = Some(format!("+{} MB", install.mb_added));
            }
            if install.tier_after > install.tier_before {
                self.tier_up = Some(install);
            }
        }
    }

    /// Back to Home, reloading it from the Api and clearing the pack state.
    pub fn go_home(&mut self) {
        self.home = self.api.home();
        self.screen = Screen::Home;
        self.detail = None;
        self.last = None;
        self.toast = None;
    }

    // -- drawing ------------------------------------------------------------

    /// Heading `Mod manager`, labels `Tier {tier}` and `{mb_into_tier} / {mb_for_next} MB`,
    /// a ProgressBar from `mb_fraction`, then the next-pack card (heading with the name,
    /// summary, `{loader}, {size_mb} MB`, and an `Install` button). With no next pack, a
    /// label `Everything installed`.
    fn home_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Mod manager");
        let s = &self.home.stats;
        ui.label(format!("Tier {}", s.tier));
        ui.label(format!("{} / {} MB", s.mb_into_tier, s.mb_for_next));
        ui.add(egui::ProgressBar::new(mb_fraction(s)));
        ui.separator();

        let mut install: Option<String> = None;
        match &self.home.next {
            Some(pack) => {
                ui.heading(pack.name.as_str());
                ui.label(pack.summary.as_str());
                ui.label(format!("{}, {} MB", pack.loader, pack.size_mb));
                if ui.button("Install").clicked() {
                    install = Some(pack.id.clone());
                }
            }
            None => {
                ui.label("Everything installed");
            }
        }
        if let Some(id) = install {
            self.open(&id);
        }
    }

    /// Heading with the pack name; a label `{verified}/{total} mods verified`; one label per
    /// mod, `[x] {name}` when its file was verified in `last`, else `[ ] {name}`; the
    /// toast if any; buttons `Verify files` and `Back`. `Back` or the Esc key (while no
    /// tier-up modal is open) calls `go_home`.
    fn pack_ui(&mut self, ui: &mut egui::Ui) {
        let (name, mods) = match &self.detail {
            Some(detail) => (detail.pack.name.clone(), detail.pack.mods.clone()),
            None => return,
        };

        ui.heading(name.as_str());
        let verified = match &self.last {
            Some(result) => verified_count(result),
            None => 0,
        };
        ui.label(format!("{verified}/{} mods verified", mods.len()));
        for m in &mods {
            let done = match &self.last {
                Some(result) => result
                    .files
                    .iter()
                    .any(|f| f.file == m.file && f.status == FileStatus::Verified),
                None => false,
            };
            let mark = if done { "[x]" } else { "[ ]" };
            ui.label(format!("{mark} {}", m.name));
        }
        if let Some(toast) = &self.toast {
            ui.label(toast.as_str());
        }

        let verify = ui.button("Verify files").clicked();
        let back = ui.button("Back").clicked();
        let esc = self.tier_up.is_none() && ui.input(|i| i.key_pressed(egui::Key::Escape));
        if verify {
            self.verify();
        }
        if back || esc {
            self.go_home();
        }
    }

    /// A modal with the heading `Tier up!`, the label `You reached tier {tier_after}`
    /// and a `Continue` button. Clears `tier_up` when Continue is clicked or the modal
    /// asks to close (`response.should_close()`).
    fn tier_up_modal(&mut self, ctx: &egui::Context) {
        let tier = match &self.tier_up {
            Some(install) => install.tier_after,
            None => return,
        };
        let mut close = false;
        let response = egui::Modal::new(egui::Id::new("tier_up")).show(ctx, |ui| {
            ui.heading("Tier up!");
            ui.label(format!("You reached tier {tier}"));
            if ui.button("Continue").clicked() {
                close = true;
            }
        });
        if close || response.should_close() {
            self.tier_up = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn pack() -> Modpack {
        Modpack {
            id: "fabulously-optimized".into(),
            name: "Fabulously Optimized".into(),
            summary: "Faster frames with a handful of mods.".into(),
            size_mb: 40,
            loader: "fabric".into(),
            mods: vec![
                PackMod {
                    file: "sodium.jar".into(),
                    name: "Sodium".into(),
                },
                PackMod {
                    file: "lithium.jar".into(),
                    name: "Lithium".into(),
                },
            ],
        }
    }

    fn stats(tier: u32, into: u32) -> LibraryStats {
        LibraryStats {
            mb: 140,
            tier,
            mb_into_tier: into,
            mb_for_next: 100,
            packs_installed: 2,
        }
    }

    fn result(first: FileStatus, second: FileStatus) -> VerifyResult {
        let p = pack();
        let statuses = [first, second];
        VerifyResult {
            ok: statuses.iter().all(|s| *s == FileStatus::Verified),
            downloaded: true,
            files: p
                .mods
                .iter()
                .zip(statuses)
                .map(|(m, status)| FileResult {
                    file: m.file.clone(),
                    name: m.name.clone(),
                    status,
                    output: None,
                })
                .collect(),
            duration_ms: 12,
        }
    }

    fn install(tier_before: u32, tier_after: u32) -> Install {
        Install {
            pack_id: pack().id,
            mb_added: 50,
            tier_before,
            tier_after,
        }
    }

    /// Scripted Api: answers with canned data and records every call.
    struct FakeApi {
        outcomes: RefCell<VecDeque<VerifyOutcome>>,
        calls: RefCell<Vec<String>>,
    }

    impl FakeApi {
        fn new(outcomes: Vec<VerifyOutcome>) -> Self {
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
                next: Some(pack()),
                stats: stats(1, 40),
            }
        }

        fn open_pack(&self, id: &str) -> PackDetail {
            self.calls.borrow_mut().push(format!("open:{id}"));
            PackDetail {
                pack: pack(),
                notes: "Sodium and Lithium.".into(),
                installed: false,
                last_verify: None,
            }
        }

        fn verify(&self, id: &str) -> VerifyOutcome {
            self.calls.borrow_mut().push(format!("verify:{id}"));
            self.outcomes
                .borrow_mut()
                .pop_front()
                .expect("the test did not script another verify result")
        }
    }

    fn harness(outcomes: Vec<VerifyOutcome>) -> Harness<'static, ModManagerApp<FakeApi>> {
        Harness::builder().with_max_steps(30).build_ui_state(
            |ui, app: &mut ModManagerApp<FakeApi>| app.ui(ui),
            ModManagerApp::new(FakeApi::new(outcomes)),
        )
    }

    fn broken() -> VerifyOutcome {
        VerifyOutcome {
            result: result(FileStatus::Broken, FileStatus::NotChecked),
            install: None,
        }
    }

    fn clean(install: Install) -> VerifyOutcome {
        VerifyOutcome {
            result: result(FileStatus::Verified, FileStatus::Verified),
            install: Some(install),
        }
    }

    #[test]
    fn mb_fraction_is_progress_through_tier() {
        assert_eq!(mb_fraction(&stats(1, 25)), 0.25);
        assert_eq!(mb_fraction(&stats(1, 0)), 0.0);
        assert_eq!(mb_fraction(&stats(1, 100)), 1.0);
        let mut s = stats(1, 10);
        s.mb_for_next = 0;
        assert_eq!(mb_fraction(&s), 0.0, "no division by zero");
        assert_eq!(mb_fraction(&stats(1, 250)), 1.0, "never above 1");
    }

    #[test]
    fn home_shows_next_pack_card() {
        let harness = harness(vec![]);
        harness.get_by_label("Mod manager");
        harness.get_by_label("Tier 1");
        harness.get_by_label("40 / 100 MB");
        harness.get_by_label("Fabulously Optimized");
        harness.get_by_label("Faster frames with a handful of mods.");
        harness.get_by_label("fabric, 40 MB");
        harness.get_by_label("Install");
    }

    #[test]
    fn install_opens_the_pack_screen() {
        let mut harness = harness(vec![]);
        harness.get_by_label("Install").click();
        harness.run();
        harness.get_by_label("Fabulously Optimized");
        harness.get_by_label("0/2 mods verified");
        harness.get_by_label("[ ] Sodium");
        harness.get_by_label("[ ] Lithium");
        assert!(
            harness
                .state()
                .api()
                .calls
                .borrow()
                .contains(&"open:fabulously-optimized".to_string()),
            "Install must call Api::open_pack with the pack id"
        );
    }

    #[test]
    fn broken_verify_shows_zero_of_n() {
        let mut harness = harness(vec![broken()]);
        harness.get_by_label("Install").click();
        harness.run();
        harness.get_by_label("Verify files").click();
        harness.run();
        harness.get_by_label("0/2 mods verified");
        harness.get_by_label("[ ] Sodium");
        assert!(harness.query_by_label("+50 MB").is_none(), "no toast on red");
        assert!(harness.query_by_label("Tier up!").is_none());
    }

    #[test]
    fn clean_verify_ticks_all_and_shows_mb_toast() {
        let mut harness = harness(vec![clean(install(1, 1))]);
        harness.get_by_label("Install").click();
        harness.run();
        harness.get_by_label("Verify files").click();
        harness.run();
        harness.get_by_label("2/2 mods verified");
        harness.get_by_label("[x] Sodium");
        harness.get_by_label("[x] Lithium");
        harness.get_by_label("+50 MB");
        assert!(
            harness.query_by_label("Tier up!").is_none(),
            "same tier: no modal"
        );
    }

    #[test]
    fn tier_up_shows_modal_until_continue() {
        let mut harness = harness(vec![clean(install(1, 2))]);
        harness.get_by_label("Install").click();
        harness.run();
        harness.get_by_label("Verify files").click();
        harness.run();
        harness.get_by_label("Tier up!");
        harness.get_by_label("You reached tier 2");
        harness.get_by_label("Continue").click();
        harness.run();
        assert!(harness.query_by_label("Tier up!").is_none(), "Continue closes it");
        harness.get_by_label("+50 MB");
    }

    #[test]
    fn escape_returns_home() {
        let mut harness = harness(vec![]);
        harness.get_by_label("Install").click();
        harness.run();
        harness.get_by_label("Verify files");
        harness.key_press(egui::Key::Escape);
        harness.run();
        harness.get_by_label("Install");
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
