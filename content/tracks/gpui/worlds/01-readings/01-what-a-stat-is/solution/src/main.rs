use kinetik_01_what_a_stat_is::ALL;

fn main() {
    for kind in ALL {
        println!("{:>4}  {kind:?}", kind.label());
    }
}
