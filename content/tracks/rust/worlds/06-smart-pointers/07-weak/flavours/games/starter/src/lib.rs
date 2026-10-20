use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A power pole. It owns the machines it supplies.
pub struct PowerPole {
    pub name: String,
    pub machines: RefCell<Vec<Rc<Machine>>>,
}

/// A machine that can look up its pole, without keeping it alive.
pub struct Machine {
    pub name: String,
    pub pole: RefCell<Weak<PowerPole>>,
}

pub fn new_pole(name: &str) -> Rc<PowerPole> {
    Rc::new(PowerPole {
        name: name.to_string(),
        machines: RefCell::new(Vec::new()),
    })
}

pub fn new_machine(name: &str) -> Rc<Machine> {
    Rc::new(Machine {
        name: name.to_string(),
        pole: RefCell::new(Weak::new()),
    })
}

/// Puts `machine` on `pole`. The machine's back-pointer must be Weak.
pub fn connect(pole: &Rc<PowerPole>, machine: &Rc<Machine>) {
    // TODO: the pole gets a (strong) machine in its list; the machine gets a weak link back to the pole
    todo!()
}

/// The pole's name, or None if the machine has no pole (or the pole was removed).
pub fn pole_name(machine: &Machine) -> Option<String> {
    // TODO: the pole's name, if the pole is still alive
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_links_pole_and_machine() {
        let pole = new_pole("pole-north");
        let furnace = new_machine("furnace");
        assert_eq!(pole_name(&furnace), None, "a fresh machine has no pole");
        connect(&pole, &furnace);
        assert_eq!(pole.machines.borrow().len(), 1);
        assert_eq!(pole_name(&furnace), Some(String::from("pole-north")));
    }

    #[test]
    fn pole_link_is_weak() {
        let pole = new_pole("pole-north");
        let furnace = new_machine("furnace");
        connect(&pole, &furnace);
        assert_eq!(Rc::strong_count(&pole), 1, "a machine must not own its pole");
        assert_eq!(Rc::weak_count(&pole), 1);
        assert_eq!(Rc::strong_count(&furnace), 2, "pole owns machine: furnace + the Vec");
    }

    #[test]
    fn pole_name_none_after_pole_dropped() {
        let furnace = new_machine("furnace");
        {
            let pole = new_pole("pole-north");
            connect(&pole, &furnace);
            assert_eq!(pole_name(&furnace), Some(String::from("pole-north")));
        }
        assert_eq!(pole_name(&furnace), None, "pole is gone, upgrade() must give None");
        assert_eq!(Rc::strong_count(&furnace), 1, "the pole's Vec released its handle");
    }
}
