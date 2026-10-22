use kinetik_01_boss_stat_model::{BarItem, Column, Limits, Snapshot, StatKind, bar_items, default_order, two_line_columns};

fn main() {
    let snapshot = Snapshot::from([
        (StatKind::CpuUsage, 12.6),
        (StatKind::CpuTemp, 90.0),
        (StatKind::GpuUsage, 41.0),
        (StatKind::GpuTemp, 64.0),
        (StatKind::RamUsed, 8_589_934_592.0),
        (StatKind::NetDown, 3_565_158.0),
        (StatKind::NetUp, 52_000.0),
    ]);
    let items = bar_items(&default_order(), &snapshot, &Limits::default());
    let one_line: Vec<String> = items
        .iter()
        .map(|item| match item {
            BarItem::Pill(p) => format!("{} {}{}", p.label, p.text, if p.warning { "!" } else { "" }),
            BarItem::Divider(_) => "|".to_string(),
        })
        .collect();
    println!("{}\n", one_line.join("  "));
    for column in two_line_columns(&items) {
        match column {
            Column::Pills(top, bottom) => {
                let under = bottom.map(|b| format!("{} {}", b.label, b.text)).unwrap_or_default();
                println!("{:<14}/ {under}", format!("{} {}", top.label, top.text));
            }
            Column::Divider(_) => println!("|"),
        }
    }
}
