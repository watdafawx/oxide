/// How full the XP bar is: `xp_into_level / xp_for_next`, always between 0.0 and 1.0.
/// An `xp_for_next` of 0 gives 0.0.
pub fn xp_fraction(xp_into_level: u32, xp_for_next: u32) -> f32 {
    // TODO: 0.0 when xp_for_next is 0; otherwise divide the two as f32 numbers and keep
    //       the answer between 0.0 and 1.0
    todo!()
}

/// The XP bar under the level badge.
pub struct XpBar {
    pub xp_into_level: u32,
    pub xp_for_next: u32,
}

impl XpBar {
    /// A label like `40 / 100 XP`, then a progress bar filled to `xp_fraction`.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // TODO: draw the label, then add an egui::ProgressBar built from the fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    #[test]
    fn fraction_is_progress_through_the_level() {
        assert_eq!(xp_fraction(25, 100), 0.25);
        assert_eq!(xp_fraction(0, 100), 0.0);
        assert_eq!(xp_fraction(100, 100), 1.0);
        assert_eq!(xp_fraction(250, 100), 1.0, "never above 1");
        assert_eq!(xp_fraction(10, 0), 0.0, "no division by zero");
    }

    #[test]
    fn the_bar_screen_shows_text_and_bar() {
        let bar = XpBar {
            xp_into_level: 40,
            xp_for_next: 100,
        };
        let harness = Harness::new_ui_state(|ui, bar: &mut XpBar| bar.ui(ui), bar);
        harness.get_by_label("40 / 100 XP");
        harness.get_by_role(egui::accesskit::Role::ProgressIndicator);
    }
}
