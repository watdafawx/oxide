use coll_boss_csv_stats::*;
use std::panic;

const CSV: &str = "name,loader,downloads,size_mb
sodium,fabric,9000,1.2
lithium,fabric,6000,0.5
jei,forge,12000,3.4
create,forge,8000,18.0
ae2,neoforge,4000,9.5
";

fn main() {
    println!("== Mod list stats ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let mods = match parse_modlist(CSV) {
            Ok(mods) => mods,
            Err(e) => {
                println!("Could not read the mod list: {e}");
                return;
            }
        };
        println!("{}", report(&mods));
        println!();
        println!("Downloads by loader:");
        for (loader, total) in downloads_by_loader(&mods) {
            println!("  {loader}: {total}");
        }
        println!("Top 2 mods:");
        for m in top_n(&mods, 2) {
            println!("  {} ({})", m.name, m.downloads);
        }
        match smallest_in(&mods, "forge") {
            Some(m) => println!("Smallest forge mod: {} at {:.1} MB ({:.0} KB)", m.name, m.size_mb, m.size_kb()),
            None => println!("No forge mods."),
        }
        println!();
        println!("A broken file: {:?}", parse_modlist("name,loader,downloads,size_mb\nsodium,fabric,lots,1\n"));
    });
    if result.is_err() {
        println!("Finish `parse_mod`, `parse_modlist`, `downloads_by_loader`, `top_n`, `smallest_in` and `report` to see the stats.");
    }
}
