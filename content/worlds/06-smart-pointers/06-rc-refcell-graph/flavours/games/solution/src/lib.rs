use std::cell::RefCell;
use std::rc::Rc;

/// A machine with a rate (items per minute) and the machines it feeds.
pub struct Assembler {
    pub rate: u32,
    pub feeds: Vec<Rc<RefCell<Assembler>>>,
}

/// A new assembler that feeds nothing yet.
pub fn new_assembler(rate: u32) -> Rc<RefCell<Assembler>> {
    Rc::new(RefCell::new(Assembler {
        rate,
        feeds: Vec::new(),
    }))
}

/// Makes `source` feed `target`.
pub fn connect(source: &Rc<RefCell<Assembler>>, target: Rc<RefCell<Assembler>>) {
    source.borrow_mut().feeds.push(target);
}

/// The assembler's rate plus the throughput of everything downstream of it.
pub fn total_throughput(assembler: &Rc<RefCell<Assembler>>) -> u32 {
    let a = assembler.borrow();
    a.rate + a.feeds.iter().map(total_throughput).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_is_a_leaf() {
        let a = new_assembler(5);
        assert_eq!(a.borrow().rate, 5);
        assert_eq!(a.borrow().feeds.len(), 0);
    }

    #[test]
    fn add_child_links_parent() {
        let gears = new_assembler(1);
        connect(&gears, new_assembler(2));
        assert_eq!(gears.borrow().feeds.len(), 1);
        assert_eq!(gears.borrow().feeds[0].borrow().rate, 2);
    }

    #[test]
    fn shared_child_changes_show_up() {
        let gears = new_assembler(1);
        let circuits = new_assembler(2);
        connect(&gears, Rc::clone(&circuits));
        circuits.borrow_mut().rate = 10;
        assert_eq!(
            total_throughput(&gears),
            11,
            "the line should see the edit made through `circuits`"
        );
    }

    #[test]
    fn total_sums_whole_tree() {
        let gears = new_assembler(1);
        let circuits = new_assembler(2);
        let science = new_assembler(3);
        connect(&circuits, new_assembler(4));
        connect(&gears, circuits);
        connect(&gears, science);
        assert_eq!(total_throughput(&gears), 10);
    }
}
