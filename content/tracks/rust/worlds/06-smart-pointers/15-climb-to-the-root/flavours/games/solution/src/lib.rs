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

/// Names from the starting tech down to `tech`.
pub fn research_path(tech: &TechRef) -> Vec<String> {
    let mut path = Vec::new();
    let mut current = Some(Rc::clone(tech));
    while let Some(t) = current {
        path.push(t.borrow().name.clone());
        current = t.borrow().prerequisite.upgrade();
    }
    path.reverse();
    path
}

/// True when some tech above `tech` (not the tech itself) has this name.
pub fn needs(tech: &TechRef, name: &str) -> bool {
    let mut current = tech.borrow().prerequisite.upgrade();
    while let Some(t) = current {
        if t.borrow().name == name {
            return true;
        }
        current = t.borrow().prerequisite.upgrade();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_path_lists_start_down_to_tech() {
        let automation = starting_tech("Automation");
        let logistics = unlock(&automation, "Logistics");
        let fast = unlock(&logistics, "Fast inserter");
        assert_eq!(research_path(&fast), vec!["Automation", "Logistics", "Fast inserter"]);
        assert_eq!(research_path(&automation), vec!["Automation"], "the start needs only itself");
    }

    #[test]
    fn needs_finds_prerequisites_above() {
        let automation = starting_tech("Automation");
        let logistics = unlock(&automation, "Logistics");
        let fast = unlock(&logistics, "Fast inserter");
        let electronics = unlock(&automation, "Electronics");
        assert!(needs(&fast, "Automation"));
        assert!(needs(&fast, "Logistics"));
        assert!(!needs(&fast, "Fast inserter"), "a tech does not need itself");
        assert!(!needs(&electronics, "Logistics"), "Logistics is a sibling, not a prerequisite");
        assert!(!needs(&automation, "Automation"), "the start needs nothing");
    }
}
