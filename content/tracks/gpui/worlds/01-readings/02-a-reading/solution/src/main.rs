use kinetik_01_a_reading::{Reading, StatKind};

fn main() {
    let read = Reading { kind: StatKind::GpuTemp, value: Some(64.0) };
    let missing = Reading { kind: StatKind::CpuTemp, value: None };
    println!("GPU temp: {}", read.text());
    println!("CPU temp: {}  (needs administrator)", missing.text());
}
