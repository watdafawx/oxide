use std::cell::RefCell;
use std::rc::Rc;

/// A shared, changeable assembler: several owners, retuned through any of them.
pub type AssemblerRef = Rc<RefCell<Assembler>>;

/// A machine with a rate (items per minute) and the machines it feeds.
pub struct Assembler {
    pub rate: u32,
    pub feeds: Vec<AssemblerRef>,
}

/// A new assembler that feeds nothing yet.
pub fn new_assembler(rate: u32) -> AssemblerRef {
    Rc::new(RefCell::new(Assembler {
        rate,
        feeds: Vec::new(),
    }))
}

/// Makes `source` feed `target`.
pub fn connect(source: &AssemblerRef, target: AssemblerRef) {
    source.borrow_mut().feeds.push(target);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_assembler_feeds_nothing() {
        let a = new_assembler(5);
        assert_eq!(a.borrow().rate, 5);
        assert_eq!(a.borrow().feeds.len(), 0);
    }

    #[test]
    fn connect_appends_to_feeds() {
        let gears = new_assembler(1);
        connect(&gears, new_assembler(2));
        assert_eq!(gears.borrow().feeds.len(), 1);
        assert_eq!(gears.borrow().feeds[0].borrow().rate, 2);
    }

    #[test]
    fn retune_through_any_handle_shows_up() {
        let gears = new_assembler(1);
        let circuits = new_assembler(2);
        connect(&gears, Rc::clone(&circuits));
        circuits.borrow_mut().rate = 10;
        assert_eq!(
            gears.borrow().feeds[0].borrow().rate,
            10,
            "the line should see the change made through `circuits`"
        );
    }
}
