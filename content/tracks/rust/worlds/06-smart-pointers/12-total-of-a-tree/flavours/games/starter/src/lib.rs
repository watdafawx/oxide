use std::cell::RefCell;
use std::rc::Rc;

pub type AssemblerRef = Rc<RefCell<Assembler>>;

/// A machine with a rate (items per minute) and the machines it feeds.
pub struct Assembler {
    pub rate: u32,
    pub feeds: Vec<AssemblerRef>,
}

pub fn new_assembler(rate: u32) -> AssemblerRef {
    Rc::new(RefCell::new(Assembler {
        rate,
        feeds: Vec::new(),
    }))
}

pub fn connect(source: &AssemblerRef, target: AssemblerRef) {
    source.borrow_mut().feeds.push(target);
}

/// The machine's rate plus the rate of every machine downstream of it.
pub fn total_throughput(assembler: &AssemblerRef) -> u32 {
    // TODO: borrow the machine; start from its rate and add total_throughput of each machine it feeds
    todo!()
}

/// How many machines: this one and every machine downstream of it.
pub fn machine_count(assembler: &AssemblerRef) -> u32 {
    // TODO: same shape as total_throughput: 1 for this machine, plus machine_count of each one it feeds
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// gears (1)
    ///   circuits (2)
    ///     science (4)
    ///   belts (3)
    fn line() -> AssemblerRef {
        let gears = new_assembler(1);
        let circuits = new_assembler(2);
        connect(&circuits, new_assembler(4));
        connect(&gears, circuits);
        connect(&gears, new_assembler(3));
        gears
    }

    #[test]
    fn total_throughput_adds_the_whole_line() {
        assert_eq!(total_throughput(&line()), 10, "1 + 2 + 4 + 3");
        assert_eq!(total_throughput(&new_assembler(7)), 7, "a machine that feeds nothing is just its own rate");
    }

    #[test]
    fn machine_count_counts_every_machine() {
        assert_eq!(machine_count(&line()), 4);
        assert_eq!(machine_count(&new_assembler(7)), 1);
    }
}
