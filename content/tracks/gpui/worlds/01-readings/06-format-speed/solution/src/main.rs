use kinetik_01_format_speed::{Reading, StatKind};

fn main() {
    for (kind, value) in [(StatKind::NetDown, 3_565_158.0), (StatKind::NetUp, 52_000.0)] {
        println!("{}", Reading { kind, value: Some(value) }.text());
    }
}
