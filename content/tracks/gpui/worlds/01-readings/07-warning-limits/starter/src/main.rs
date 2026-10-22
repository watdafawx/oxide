use kinetik_01_warning_limits::{Limits, Reading, StatKind, is_warning};

fn main() {
    let limits = Limits::default();
    for (kind, value) in [(StatKind::CpuTemp, 90.0), (StatKind::GpuTemp, 64.0), (StatKind::Ping, 140.0)] {
        let r = Reading { kind, value: Some(value) };
        let mark = if is_warning(&r, &limits) { "  <- warning" } else { "" };
        println!("{:>4} {}{mark}", kind.label(), r.text());
    }
}
