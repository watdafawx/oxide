/// The view model for the render-distance picker: the state and its rules, no drawing.
#[derive(Debug)]
pub struct RenderDistance {
    pub chunks: u32,
}

impl Default for RenderDistance {
    fn default() -> Self {
        RenderDistance { chunks: 12 }
    }
}

impl RenderDistance {
    pub fn farther(&mut self) {
        self.chunks += 1;
    }

    /// Never goes below 2.
    pub fn closer(&mut self) {
        if self.chunks > 2 {
            self.chunks -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.chunks = 12;
    }

    /// The label's text, e.g. `Render distance: 12`.
    pub fn label(&self) -> String {
        format!("Render distance: {}", self.chunks)
    }

    /// Draws `-`, the label and `+` in a row, and a `Reset` button below it.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.closer();
            }
            ui.label(self.label());
            if ui.button("+").clicked() {
                self.farther();
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

    fn harness() -> Harness<'static, RenderDistance> {
        Harness::new_ui_state(|ui, d: &mut RenderDistance| d.ui(ui), RenderDistance::default())
    }

    #[test]
    fn plus_button_raises_the_distance() {
        let mut harness = harness();
        harness.get_by_label("Render distance: 12");
        harness.get_by_label("+").click();
        harness.run();
        harness.get_by_label("Render distance: 13");
        assert_eq!(harness.state().chunks, 13);
    }

    #[test]
    fn minus_button_never_goes_below_two() {
        let mut harness = harness();
        for _ in 0..12 {
            harness.get_by_label("-").click();
            harness.run();
        }
        harness.get_by_label("Render distance: 2");
    }

    #[test]
    fn reset_button_goes_back_to_twelve() {
        let mut harness = harness();
        for _ in 0..2 {
            harness.get_by_label("+").click();
            harness.run();
        }
        harness.get_by_label("Render distance: 14");
        harness.get_by_label("Reset").click();
        harness.run();
        harness.get_by_label("Render distance: 12");
    }
}
