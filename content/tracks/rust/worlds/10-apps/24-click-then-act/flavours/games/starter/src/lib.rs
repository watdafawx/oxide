/// The mod list: one button per mod, and the mod that is selected.
pub struct ModList {
    pub names: Vec<String>,
    pub selected: Option<String>,
    /// How many times a mod was selected.
    pub selections: u32,
}

impl ModList {
    pub fn new(names: &[&str]) -> Self {
        ModList {
            names: names.iter().map(|n| n.to_string()).collect(),
            selected: None,
            selections: 0,
        }
    }

    /// Selects a mod: remembers its name and counts the selection.
    pub fn select(&mut self, name: &str) {
        self.selected = Some(name.to_string());
        self.selections += 1;
    }

    /// `Selected: <name>` (or `Pick a mod`), then one button per mod.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match &self.selected {
            Some(name) => {
                ui.label(format!("Selected: {name}"));
            }
            None => {
                ui.label("Pick a mod");
            }
        }
        // TODO: make a local Option<String> for the clicked name here
        for name in &self.names {
            if ui.button(name.as_str()).clicked() {
                // TODO: remember a copy of this name (calling select here does not compile)
            }
        }
        // TODO: after the loop, select the remembered name with self.select
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    fn harness() -> Harness<'static, ModList> {
        Harness::new_ui_state(
            |ui, list: &mut ModList| list.ui(ui),
            ModList::new(&["iris", "sodium", "lithium"]),
        )
    }

    #[test]
    fn clicking_a_mod_selects_it() {
        let mut harness = harness();
        harness.get_by_label("Pick a mod");
        harness.get_by_label("sodium").click();
        harness.run();
        harness.get_by_label("Selected: sodium");
        assert_eq!(harness.state().selections, 1, "one click selects it once");
    }

    #[test]
    fn clicking_another_switches_to_it() {
        let mut harness = harness();
        harness.get_by_label("sodium").click();
        harness.run();
        harness.get_by_label("iris").click();
        harness.run();
        harness.get_by_label("Selected: iris");
        assert_eq!(harness.state().selections, 2);
    }
}
