/// The view model: all the state and logic, no drawing.
#[derive(Debug, Default)]
pub struct ModCounter {
    pub count: u32,
}

impl ModCounter {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Never goes below 0.
    pub fn decrement(&mut self) {
        self.count = self.count.saturating_sub(1);
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    /// The text of the label, e.g. `Mods enabled: 3`.
    pub fn label(&self) -> String {
        format!("Mods enabled: {}", self.count)
    }

    /// Draws `-`, the label and `+` in a row, and a `Reset` button below.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.decrement();
            }
            ui.label(self.label());
            if ui.button("+").clicked() {
                self.increment();
            }
        });
        if ui.button("Reset").clicked() {
            self.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn harness() -> Harness<'static, ModCounter> {
        Harness::new_ui_state(|ui, counter: &mut ModCounter| counter.ui(ui), ModCounter::default())
    }

    #[test]
    fn model_enables_and_disables_mods() {
        let mut c = ModCounter::default();
        assert_eq!(c.label(), "Mods enabled: 0");
        c.increment();
        c.increment();
        assert_eq!(c.label(), "Mods enabled: 2");
        c.decrement();
        assert_eq!(c.count, 1);
        c.decrement();
        c.decrement();
        assert_eq!(c.count, 0, "decrement must stop at 0");
        c.increment();
        c.reset();
        assert_eq!(c.count, 0);
    }

    #[test]
    fn plus_button_enables_one_more_mod() {
        let mut harness = harness();
        harness.get_by_label("Mods enabled: 0");
        harness.get_by_label("+").click();
        harness.run();
        harness.get_by_label("Mods enabled: 1");
        assert_eq!(harness.state().count, 1);
    }

    #[test]
    fn reset_button_disables_every_mod() {
        let mut harness = harness();
        for _ in 0..3 {
            harness.get_by_label("+").click();
            harness.run();
        }
        harness.get_by_label("Mods enabled: 3");
        harness.get_by_label("Reset").click();
        harness.run();
        harness.get_by_label("Mods enabled: 0");
    }
}
