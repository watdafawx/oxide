use kinetik_01_move_a_slot::{Slot, default_order, move_up};

fn show(order: &[Slot]) {
    let labels: Vec<&str> = order
        .iter()
        .map(|s| match s {
            Slot::Stat(k) => k.label(),
            Slot::Divider(_) => "|",
        })
        .collect();
    println!("{}", labels.join(" "));
}

fn main() {
    let mut order = default_order();
    show(&order);
    move_up(&mut order, 3); // GPU usage one step left
    show(&order);
}
