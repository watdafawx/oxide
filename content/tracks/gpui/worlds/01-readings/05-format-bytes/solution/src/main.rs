use kinetik_01_format_bytes::{Reading, StatKind};

fn main() {
    let ram = Reading { kind: StatKind::RamUsed, value: Some(8_589_934_592.0) };
    println!(" RAM {}", ram.text());
}
