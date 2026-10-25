use kinetik_01_two_lines::{BarItem, Column, DividerStyle, Pill, StatKind, two_line_columns};

fn main() {
    let pill = |kind: StatKind, text: &str| BarItem::Pill(Pill { kind, label: kind.label(), text: text.to_string(), warning: false });
    let items = vec![
        pill(StatKind::CpuUsage, "13%"),
        pill(StatKind::CpuTemp, "47°C"),
        BarItem::Divider(DividerStyle::Line),
        pill(StatKind::RamUsed, "8.0 GB"),
        BarItem::Divider(DividerStyle::Line),
        pill(StatKind::NetDown, "↓ 3.4 MB/s"),
        pill(StatKind::NetUp, "↑ 50.8 KB/s"),
        pill(StatKind::Ping, "20 ms"),
    ];
    for column in two_line_columns(&items) {
        match column {
            Column::Pills(top, bottom) => {
                let under = bottom.map(|b| format!("{} {}", b.label, b.text)).unwrap_or_default();
                println!("{:<16}/ {under}", format!("{} {}", top.label, top.text));
            }
            Column::Divider(_) => println!("|"),
        }
    }
}
