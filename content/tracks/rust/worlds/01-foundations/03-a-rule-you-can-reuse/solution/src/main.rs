use found_a_rule_you_can_reuse::{boxes, sheets};

fn main() {
    for crates in [12, 7, 100] {
        let s = sheets(crates);
        println!("{crates} crates -> {s} sheets -> {} boxes", boxes(s));
    }
}
