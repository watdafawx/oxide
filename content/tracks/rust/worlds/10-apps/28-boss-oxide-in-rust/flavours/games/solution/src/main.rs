use app_boss_oxide_in_rust::*;
use std::cell::Cell;
use std::panic;

/// A pretend mod manager engine: every pack has two mods, and the first verify finds one broken.
struct FakeApi {
    mb: Cell<u32>,
    verifies: Cell<u32>,
}

fn pack() -> Modpack {
    Modpack {
        id: "demo".into(),
        name: "Better Frames".into(),
        summary: "Two mods that make the game run faster.".into(),
        size_mb: 50,
        loader: "fabric".into(),
        mods: vec![
            PackMod { file: "sodium.jar".into(), name: "Sodium".into() },
            PackMod { file: "lithium.jar".into(), name: "Lithium".into() },
        ],
    }
}

impl FakeApi {
    fn stats(&self) -> LibraryStats {
        let mb = self.mb.get();
        LibraryStats { mb, tier: mb / 100 + 1, mb_into_tier: mb % 100, mb_for_next: 100, packs_installed: 3 }
    }
}

impl Api for FakeApi {
    fn home(&self) -> Home {
        Home { next: Some(pack()), stats: self.stats() }
    }
    fn open_pack(&self, _id: &str) -> PackDetail {
        PackDetail { pack: pack(), notes: String::new(), installed: false, last_verify: None }
    }
    fn verify(&self, _id: &str) -> VerifyOutcome {
        let n = self.verifies.get();
        self.verifies.set(n + 1);
        let all = n > 0;
        let files = pack()
            .mods
            .into_iter()
            .enumerate()
            .map(|(i, m)| FileResult {
                file: m.file,
                name: m.name,
                status: if all || i == 0 { FileStatus::Verified } else { FileStatus::Broken },
                output: None,
            })
            .collect();
        let before = self.stats().tier;
        let install = all.then(|| {
            self.mb.set(self.mb.get() + 50);
            Install { pack_id: "demo".into(), mb_added: 50, tier_before: before, tier_after: self.stats().tier }
        });
        VerifyOutcome { result: VerifyResult { ok: all, downloaded: true, files, duration_ms: 120 }, install }
    }
}

fn show(app: &ModManagerApp<FakeApi>) {
    println!(
        "  screen {:?}, tier bar {:.0}%, mods verified {}, toast {:?}, tier-up {}",
        app.screen,
        mb_fraction(&app.home.stats) * 100.0,
        match &app.last {
            Some(result) => verified_count(result),
            None => 0,
        },
        app.toast,
        app.tier_up.is_some(),
    );
}

fn main() {
    println!("== Mod manager in Rust ==");
    println!("Driving the app state with a fake engine (no window; the egui drawing is for the tests).");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let mut app = ModManagerApp::new(FakeApi { mb: Cell::new(80), verifies: Cell::new(0) });
        println!("Home:");
        show(&app);
        println!("Open the pack:");
        app.open("demo");
        show(&app);
        println!("Verify files (one mod is broken):");
        app.verify();
        show(&app);
        println!("Verify again (all good, tier up):");
        app.verify();
        show(&app);
        println!("Back to Home:");
        app.go_home();
        show(&app);
    });
    if result.is_err() {
        println!("Finish `mb_fraction`, `verified_count`, `open`, `verify` and `go_home` to drive the app.");
    }
}
