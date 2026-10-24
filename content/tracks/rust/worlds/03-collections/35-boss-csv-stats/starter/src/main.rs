use coll_boss_csv_stats::*;
use std::panic;

const CSV: &str = "name,category,price,qty
pen,office,1.50,10
stapler,office,7.25,2
apple,food,0.40,50
bread,food,2.10,6
lamp,home,19.99,3
";

fn main() {
    println!("== CSV stats ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let rows = match parse_rows(CSV) {
            Ok(rows) => rows,
            Err(e) => {
                println!("Could not read the CSV: {e}");
                return;
            }
        };
        println!("{}", report(&rows));
        println!();
        println!("Totals by category:");
        for (category, total) in total_by_category(&rows) {
            println!("  {category}: {total:.2}");
        }
        println!("Top 2 lines:");
        for row in top_n(&rows, 2) {
            println!("  {} ({:.2})", row.name, row.total());
        }
        match cheapest_in(&rows, "office") {
            Some(row) => println!("Cheapest office item: {} at {:.2}", row.name, row.price),
            None => println!("No office items."),
        }
        println!();
        println!("A broken file: {:?}", parse_rows("name,category,price,qty\npen,office,abc,1\n"));
    });
    if result.is_err() {
        println!("Finish `parse_line`, `parse_rows`, `total_by_category`, `top_n`, `cheapest_in` and `report` to see the stats.");
    }
}
