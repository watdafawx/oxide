use kinetik_01_format_percent::{Reading, StatKind};

fn main() {
    for (kind, value) in [(StatKind::CpuUsage, 12.638), (StatKind::GpuUsage, 97.2), (StatKind::Battery, 15.0)] {
        let r = Reading { kind, value: Some(value) };
        println!("{:>4} {}", kind.label(), r.text());
    }
}
