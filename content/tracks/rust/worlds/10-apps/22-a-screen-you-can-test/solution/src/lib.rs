/// The view model for the daily-goal picker: the state and its rules, no drawing.
#[derive(Debug)]
pub struct DailyGoal {
    pub quests: u32,
}

impl Default for DailyGoal {
    fn default() -> Self {
        DailyGoal { quests: 3 }
    }
}

impl DailyGoal {
    pub fn more(&mut self) {
        self.quests += 1;
    }

    /// Never goes below 1.
    pub fn fewer(&mut self) {
        if self.quests > 1 {
            self.quests -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.quests = 3;
    }

    /// The label's text, e.g. `Daily goal: 3`.
    pub fn label(&self) -> String {
        format!("Daily goal: {}", self.quests)
    }

    /// Draws `-`, the label and `+` in a row, and a `Reset` button below it.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.fewer();
            }
            ui.label(self.label());
            if ui.button("+").clicked() {
                self.more();
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

    fn harness() -> Harness<'static, DailyGoal> {
        Harness::new_ui_state(|ui, goal: &mut DailyGoal| goal.ui(ui), DailyGoal::default())
    }

    #[test]
    fn plus_button_raises_the_goal() {
        let mut harness = harness();
        harness.get_by_label("Daily goal: 3");
        harness.get_by_label("+").click();
        harness.run();
        harness.get_by_label("Daily goal: 4");
        assert_eq!(harness.state().quests, 4);
    }

    #[test]
    fn minus_button_never_goes_below_one() {
        let mut harness = harness();
        for _ in 0..4 {
            harness.get_by_label("-").click();
            harness.run();
        }
        harness.get_by_label("Daily goal: 1");
    }

    #[test]
    fn reset_button_goes_back_to_three() {
        let mut harness = harness();
        for _ in 0..2 {
            harness.get_by_label("+").click();
            harness.run();
        }
        harness.get_by_label("Daily goal: 5");
        harness.get_by_label("Reset").click();
        harness.run();
        harness.get_by_label("Daily goal: 3");
    }
}
