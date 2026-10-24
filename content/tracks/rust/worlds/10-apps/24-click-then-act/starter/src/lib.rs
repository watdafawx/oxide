/// The quest list: one button per title, and the quest that is open.
pub struct QuestList {
    pub titles: Vec<String>,
    pub open: Option<String>,
    /// How many times a quest was opened.
    pub opened: u32,
}

impl QuestList {
    pub fn new(titles: &[&str]) -> Self {
        QuestList {
            titles: titles.iter().map(|t| t.to_string()).collect(),
            open: None,
            opened: 0,
        }
    }

    /// Opens a quest: remembers its title and counts the open.
    pub fn open_quest(&mut self, title: &str) {
        self.open = Some(title.to_string());
        self.opened += 1;
    }

    /// `Open: <title>` (or `Pick a quest`), then one button per title.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match &self.open {
            Some(title) => {
                ui.label(format!("Open: {title}"));
            }
            None => {
                ui.label("Pick a quest");
            }
        }
        // TODO: make a local Option<String> for the clicked title here
        for title in &self.titles {
            if ui.button(title.as_str()).clicked() {
                // TODO: remember a copy of this title (calling open_quest here does not compile)
            }
        }
        // TODO: after the loop, open the remembered title with self.open_quest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn harness() -> Harness<'static, QuestList> {
        Harness::new_ui_state(
            |ui, list: &mut QuestList| list.ui(ui),
            QuestList::new(&["Variables", "Loops", "Functions"]),
        )
    }

    #[test]
    fn clicking_a_quest_opens_it() {
        let mut harness = harness();
        harness.get_by_label("Pick a quest");
        harness.get_by_label("Loops").click();
        harness.run();
        harness.get_by_label("Open: Loops");
        assert_eq!(harness.state().opened, 1, "one click opens it once");
    }

    #[test]
    fn clicking_another_switches_to_it() {
        let mut harness = harness();
        harness.get_by_label("Loops").click();
        harness.run();
        harness.get_by_label("Variables").click();
        harness.run();
        harness.get_by_label("Open: Variables");
        assert_eq!(harness.state().opened, 2);
    }
}
