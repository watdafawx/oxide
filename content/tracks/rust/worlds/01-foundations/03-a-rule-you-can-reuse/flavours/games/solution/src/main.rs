use found_a_rule_you_can_reuse::{planks, sticks};

fn main() {
    for logs in [12, 7, 100] {
        let p = planks(logs);
        println!("{logs} logs -> {p} planks -> {} sticks", sticks(p));
    }
}
