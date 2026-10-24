use kinetik_01_no_data_yet::{History, StatKind, hover_summary};

fn main() {
    let mut cpu = History::new(60);
    for v in [4.0, 9.0, 61.0, 13.0, 7.0] {
        cpu.push(v);
    }
    println!("CPU:  {}", hover_summary(StatKind::CpuUsage, &cpu));
    println!("Ping: {}", hover_summary(StatKind::Ping, &History::new(60)));
}
