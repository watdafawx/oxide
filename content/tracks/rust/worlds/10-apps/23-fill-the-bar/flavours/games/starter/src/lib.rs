/// How full the download bar is: `done_mb / total_mb`, always between 0.0 and 1.0.
/// A `total_mb` of 0 gives 0.0.
pub fn download_fraction(done_mb: u32, total_mb: u32) -> f32 {
    // TODO: 0.0 when total_mb is 0; otherwise divide the two as f32 numbers and keep
    //       the answer between 0.0 and 1.0
    todo!()
}

/// The download bar on the modpack screen.
pub struct DownloadBar {
    pub done_mb: u32,
    pub total_mb: u32,
}

impl DownloadBar {
    /// A label like `40 / 100 MB`, then a progress bar filled to `download_fraction`.
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
    fn fraction_is_progress_through_the_download() {
        assert_eq!(download_fraction(25, 100), 0.25);
        assert_eq!(download_fraction(0, 100), 0.0);
        assert_eq!(download_fraction(100, 100), 1.0);
        assert_eq!(download_fraction(250, 100), 1.0, "never above 1");
        assert_eq!(download_fraction(10, 0), 0.0, "no division by zero");
    }

    #[test]
    fn the_bar_screen_shows_text_and_bar() {
        let bar = DownloadBar {
            done_mb: 40,
            total_mb: 100,
        };
        let harness = Harness::new_ui_state(|ui, bar: &mut DownloadBar| bar.ui(ui), bar);
        harness.get_by_label("40 / 100 MB");
        harness.get_by_role(egui::accesskit::Role::ProgressIndicator);
    }
}
