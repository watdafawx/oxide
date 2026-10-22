use kinetik_01_units::ALL;

fn main() {
    for kind in ALL {
        println!("{:>4}  {kind:?} is in {:?}", kind.label(), kind.unit());
    }
}
