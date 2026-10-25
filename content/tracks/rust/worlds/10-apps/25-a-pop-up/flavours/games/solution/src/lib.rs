/// The library corner of the screen: megabytes downloaded, and a tier-up waiting to be shown.
pub struct Library {
    pub mb: u32,
    /// The new tier, until the player dismisses the pop-up.
    pub tier_up: Option<u32>,
}

impl Library {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("{} MB", self.mb));
        if self.tier_up.is_some() {
            let ctx = ui.ctx().clone();
            self.tier_up_modal(&ctx);
        }
    }

    /// A modal with the heading `Tier up!`, the label `You reached tier N` and a
    /// `Continue` button. Clears `tier_up` on Continue or when the modal should close.
    fn tier_up_modal(&mut self, ctx: &egui::Context) {
        let tier = match self.tier_up {
            Some(tier) => tier,
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
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn harness() -> Harness<'static, Library> {
        Harness::new_ui_state(
            |ui, l: &mut Library| l.ui(ui),
            Library {
                mb: 250,
                tier_up: Some(3),
            },
        )
    }

    #[test]
    fn the_modal_shows_the_new_tier() {
        let harness = harness();
        harness.get_by_label("Tier up!");
        harness.get_by_label("You reached tier 3");
        harness.get_by_label("Continue");
    }

    #[test]
    fn continue_closes_it() {
        let mut harness = harness();
        harness.get_by_label("Continue").click();
        harness.run();
        assert!(harness.query_by_label("Tier up!").is_none(), "Continue closes it");
        assert_eq!(harness.state().tier_up, None);
        harness.get_by_label("250 MB");
    }
}
