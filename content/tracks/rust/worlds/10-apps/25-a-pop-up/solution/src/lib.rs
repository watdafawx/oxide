/// The rewards corner of the screen: your XP, and a level-up waiting to be shown.
pub struct Rewards {
    pub xp: u32,
    /// The new level, until the player dismisses the pop-up.
    pub level_up: Option<u32>,
}

impl Rewards {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("{} XP", self.xp));
        if self.level_up.is_some() {
            let ctx = ui.ctx().clone();
            self.level_up_modal(&ctx);
        }
    }

    /// A modal with the heading `Level up!`, the label `You reached level N` and a
    /// `Continue` button. Clears `level_up` on Continue or when the modal should close.
    fn level_up_modal(&mut self, ctx: &egui::Context) {
        let level = match self.level_up {
            Some(level) => level,
            None => return,
        };
        let mut close = false;
        let response = egui::Modal::new(egui::Id::new("level_up")).show(ctx, |ui| {
            ui.heading("Level up!");
            ui.label(format!("You reached level {level}"));
            if ui.button("Continue").clicked() {
                close = true;
            }
        });
        if close || response.should_close() {
            self.level_up = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn harness() -> Harness<'static, Rewards> {
        Harness::new_ui_state(
            |ui, r: &mut Rewards| r.ui(ui),
            Rewards {
                xp: 250,
                level_up: Some(3),
            },
        )
    }

    #[test]
    fn the_modal_shows_the_new_level() {
        let harness = harness();
        harness.get_by_label("Level up!");
        harness.get_by_label("You reached level 3");
        harness.get_by_label("Continue");
    }

    #[test]
    fn continue_closes_it() {
        let mut harness = harness();
        harness.get_by_label("Continue").click();
        harness.run();
        assert!(harness.query_by_label("Level up!").is_none(), "Continue closes it");
        assert_eq!(harness.state().level_up, None);
        harness.get_by_label("250 XP");
    }
}
