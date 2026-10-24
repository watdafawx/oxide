use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type TechRef = Rc<RefCell<Tech>>;

/// A tech owns the techs it unlocks and points up at its prerequisite.
pub struct Tech {
    pub name: String,
    pub prerequisite: Weak<RefCell<Tech>>,
    pub unlocks: Vec<TechRef>,
}

pub fn starting_tech(name: &str) -> TechRef {
    Rc::new(RefCell::new(Tech {
        name: name.to_string(),
        prerequisite: Weak::new(),
        unlocks: Vec::new(),
    }))
}

pub fn unlock(prerequisite: &TechRef, name: &str) -> TechRef {
    let tech = Rc::new(RefCell::new(Tech {
        name: name.to_string(),
        prerequisite: Rc::downgrade(prerequisite),
        unlocks: Vec::new(),
    }));
    prerequisite.borrow_mut().unlocks.push(Rc::clone(&tech));
    tech
}

/// True when `a` and `b` are handles to the very same tech.
pub fn is_same(a: &TechRef, b: &TechRef) -> bool {
    // TODO: do a and b point at the same tech on the heap?
    todo!()
}

/// Takes `tech` out of its prerequisite's unlocks. The starting tech is left alone.
pub fn detach(tech: &TechRef) {
    // TODO: get the prerequisite in its own statement; if there is one,
    // retain every unlock of it that is not this exact tech
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(tech: &TechRef) -> Vec<String> {
        let mut out = Vec::new();
        for t in &tech.borrow().unlocks {
            out.push(t.borrow().name.clone());
        }
        out
    }

    #[test]
    fn is_same_tells_look_alikes_apart() {
        let automation = starting_tech("Automation");
        let electronics = unlock(&automation, "Electronics");
        let modded = unlock(&automation, "Electronics");
        assert!(is_same(&electronics, &Rc::clone(&electronics)), "a clone is the same tech");
        assert!(!is_same(&electronics, &modded), "same name, different tech");
    }

    #[test]
    fn detach_removes_only_that_tech() {
        let automation = starting_tech("Automation");
        let electronics = unlock(&automation, "Electronics");
        let modded = unlock(&automation, "Electronics");
        unlock(&automation, "Logistics");
        detach(&modded);
        assert_eq!(names(&automation), vec!["Electronics", "Logistics"]);
        assert!(Rc::ptr_eq(&automation.borrow().unlocks[0], &electronics), "the first Electronics must stay");
        detach(&automation);
        assert_eq!(names(&automation), vec!["Electronics", "Logistics"], "the start has no prerequisite, nothing changes");
    }
}
