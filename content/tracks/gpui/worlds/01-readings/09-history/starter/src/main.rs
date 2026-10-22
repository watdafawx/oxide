use kinetik_01_history::History;

fn main() {
    let mut cpu = History::new(60);
    for second in 0..100 {
        cpu.push(second as f64);
    }
    println!("{} readings, oldest {:?}, newest {:?}", cpu.len(), cpu.oldest(), cpu.newest());
}
