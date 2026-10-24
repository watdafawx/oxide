use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A power pole. It owns the machines it supplies.
pub struct PowerPole {
    pub name: String,
    pub machines: RefCell<Vec<Rc<Machine>>>,
}

/// A machine that can look up its pole, without keeping it standing.
pub struct Machine {
    pub name: String,
    pub pole: Weak<PowerPole>,
}

pub fn new_pole(name: &str) -> Rc<PowerPole> {
    Rc::new(PowerPole {
        name: name.to_string(),
        machines: RefCell::new(Vec::new()),
    })
}

/// A machine on no pole at all.
pub fn loose_machine(name: &str) -> Rc<Machine> {
    Rc::new(Machine {
        name: name.to_string(),
        pole: Weak::new(),
    })
}

/// Places a new machine on `pole`. The pole owns it; it only points back.
pub fn place_machine(pole: &Rc<PowerPole>, name: &str) -> Rc<Machine> {
    let machine = Rc::new(Machine {
        name: name.to_string(),
        pole: Rc::downgrade(pole),
    });
    pole.machines.borrow_mut().push(Rc::clone(&machine));
    machine
}

/// The pole's name, or None for a loose machine or a pole that is gone.
pub fn pole_name(machine: &Machine) -> Option<String> {
    match machine.pole.upgrade() {
        Some(pole) => Some(pole.name.clone()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_machine_links_both_ways() {
        assert_eq!(pole_name(&loose_machine("chest")), None, "a loose machine has no pole");
        let pole = new_pole("pole-north");
        let furnace = place_machine(&pole, "furnace");
        assert_eq!(pole.machines.borrow().len(), 1);
        assert_eq!(pole.machines.borrow()[0].name, "furnace");
        assert_eq!(pole_name(&furnace), Some(String::from("pole-north")));
    }

    #[test]
    fn pole_link_is_weak() {
        let pole = new_pole("pole-north");
        let furnace = place_machine(&pole, "furnace");
        assert_eq!(Rc::strong_count(&pole), 1, "a machine must not own its pole");
        assert_eq!(Rc::weak_count(&pole), 1);
        assert_eq!(Rc::strong_count(&furnace), 2, "your handle + the pole's list");
    }

    #[test]
    fn pole_name_is_none_after_pole_removed() {
        let pole = new_pole("pole-north");
        let furnace = place_machine(&pole, "furnace");
        drop(pole);
        assert_eq!(pole_name(&furnace), None, "the pole is gone, upgrade() must give None");
        assert_eq!(Rc::strong_count(&furnace), 1, "the pole's list let go of its handle");
    }
}
