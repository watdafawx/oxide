use kinetik_01_bar_order::{Slot, default_order};

fn main() {
    let bar: Vec<String> = default_order()
        .iter()
        .map(|slot| match slot {
            Slot::Stat(kind) => kind.label().to_string(),
            Slot::Divider(_) => "|".to_string(),
        })
        .collect();
    println!("{}", bar.join(" "));
}
