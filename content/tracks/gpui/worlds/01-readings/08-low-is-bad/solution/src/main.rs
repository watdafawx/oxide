use kinetik_01_low_is_bad::{Limits, Reading, StatKind, is_warning};

fn main() {
    for value in [15.0, 80.0] {
        let r = Reading { kind: StatKind::Battery, value: Some(value) };
        let mark = if is_warning(&r, &Limits::default()) { "  <- warning" } else { "" };
        println!(" BAT {}{mark}", r.text());
    }
}
