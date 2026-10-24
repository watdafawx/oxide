/// The view model: all the state and logic, no drawing.
#[derive(Debug, Default)]
pub struct Counter {
    pub count: u32,
}

impl Counter {
    pub fn increment(&mut self) {
        // TODO: add 1 to the count
        todo!()
    }

    /// Never goes below 0.
    pub fn decrement(&mut self) {
        // TODO: take 1 off the count, stopping at 0 (u32 has a subtraction that saturates)
        todo!()
    }

    pub fn reset(&mut self) {
        // TODO: put the count back to zero
        todo!()
    }

    /// The text of the label, e.g. `Count: 3`.
    pub fn label(&self) -> String {
        // TODO: the text "Count: " followed by the count
        todo!()
    }

    /// Draws `-`, the label and `+` in a row, and a `Reset` button below.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // TODO: in a horizontal row: a "-" button, the label, a "+" button; below it a "Reset"
        //       button. Each button calls its method when clicked
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn harness() -> Harness<'static, Counter> {
        Harness::new_ui_state(|ui, counter: &mut Counter| counter.ui(ui), Counter::default())
    }

    #[test]
    fn model_counts_up_and_down() {
        let mut c = Counter::default();
        assert_eq!(c.label(), "Count: 0");
        c.increment();
        c.increment();
        assert_eq!(c.label(), "Count: 2");
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
    fn plus_button_updates_label() {
        let mut harness = harness();
        harness.get_by_label("Count: 0");
        harness.get_by_label("+").click();
        harness.run();
        harness.get_by_label("Count: 1");
        assert_eq!(harness.state().count, 1);
    }

    #[test]
    fn reset_button_goes_back_to_zero() {
        let mut harness = harness();
        for _ in 0..3 {
            harness.get_by_label("+").click();
            harness.run();
        }
        harness.get_by_label("Count: 3");
        harness.get_by_label("Reset").click();
        harness.run();
        harness.get_by_label("Count: 0");
    }
}
