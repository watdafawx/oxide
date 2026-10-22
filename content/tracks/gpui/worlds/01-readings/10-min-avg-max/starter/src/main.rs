use kinetik_01_min_avg_max::History;

fn main() {
    let mut cpu = History::new(60);
    for v in [4.0, 9.0, 61.0, 13.0, 7.0] {
        cpu.push(v);
    }
    println!("{:?}", cpu.summary());
}
