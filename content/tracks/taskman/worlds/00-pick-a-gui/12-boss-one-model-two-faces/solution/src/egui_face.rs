//! The list, drawn with egui.

use crate::model::{ProcessRow, SortBy, TaskList};

/// Draws one row: name, CPU and memory, left to right.
pub fn row_ui(ui: &mut egui::Ui, row: &ProcessRow) {
    ui.horizontal(|ui| {
        ui.label(row.name.as_str());
        ui.label(row.cpu.as_str());
        ui.label(row.memory.as_str());
    });
}

/// The search box, the column buttons, then one row per visible process.
pub fn list_ui(list: &mut TaskList, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let search = ui.label("Search");
        ui.text_edit_singleline(&mut list.filter).labelled_by(search.id);
    });
    ui.horizontal(|ui| {
        if ui.button("CPU").clicked() {
            list.sort = SortBy::Cpu;
        }
        if ui.button("Memory").clicked() {
            list.sort = SortBy::Memory;
        }
        if ui.button("Name").clicked() {
            list.sort = SortBy::Name;
        }
    });
    for row in list.visible() {
        row_ui(ui, &row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_processes;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn harness() -> Harness<'static, TaskList> {
        Harness::new_ui_state(|ui, list: &mut TaskList| list_ui(list, ui), TaskList::new(sample_processes()))
    }

    #[test]
    fn egui_memory_button_sorts_by_memory() {
        let mut harness = harness();
        harness.get_by_label("Memory").click();
        harness.run();
        assert_eq!(harness.state().sort, SortBy::Memory);
        let zoom = harness.get_by_label("Zoom.exe").rect();
        let explorer = harness.get_by_label("explorer.exe").rect();
        assert!(zoom.top() < explorer.top(), "Zoom.exe (250 MB) should be drawn above explorer.exe (180 MB)");
    }

    #[test]
    fn egui_shows_only_matching_rows() {
        let mut harness = harness();
        harness.state_mut().filter = "chrome".to_string();
        harness.run();
        harness.get_by_label("chrome.exe");
        harness.get_by_label("Chrome Helper.exe");
        assert!(harness.query_by_label("game.exe").is_none(), "game.exe does not match \"chrome\" and should not be drawn");
    }
}
