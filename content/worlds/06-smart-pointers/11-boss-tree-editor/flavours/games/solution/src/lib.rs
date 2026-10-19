use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// The starting tech always has this id.
pub const ROOT: u32 = 0;

type TechRef = Rc<RefCell<Tech>>;

pub struct Tech {
    pub name: String,
    pub unlocks: Vec<TechRef>,
    pub prerequisite: Weak<RefCell<Tech>>,
}

pub struct TechTree {
    techs: HashMap<u32, TechRef>,
    next_id: u32,
}

impl TechTree {
    /// A tree with only the starting tech (id 0) called `start`.
    pub fn new(start: &str) -> TechTree {
        let root = Rc::new(RefCell::new(Tech {
            name: start.to_string(),
            unlocks: Vec::new(),
            prerequisite: Weak::new(),
        }));
        let mut techs = HashMap::new();
        techs.insert(ROOT, root);
        TechTree { techs, next_id: 1 }
    }

    /// The name of tech `id`, or None if there is no such tech.
    pub fn name(&self, id: u32) -> Option<String> {
        self.techs.get(&id).map(|t| t.borrow().name.clone())
    }

    /// Adds a tech unlocked by `prerequisite`. Returns the new id, or None if `prerequisite` does not exist.
    pub fn add(&mut self, prerequisite: u32, name: &str) -> Option<u32> {
        let prerequisite_tech = self.techs.get(&prerequisite)?;
        let id = self.next_id;
        self.next_id += 1;
        let tech = Rc::new(RefCell::new(Tech {
            name: name.to_string(),
            unlocks: Vec::new(),
            prerequisite: Rc::downgrade(prerequisite_tech),
        }));
        prerequisite_tech.borrow_mut().unlocks.push(Rc::clone(&tech));
        self.techs.insert(id, tech);
        Some(id)
    }

    /// Names from the starting tech down to tech `id`. Empty if `id` does not exist.
    pub fn path(&self, id: u32) -> Vec<String> {
        let mut out = Vec::new();
        let mut current = self.techs.get(&id).cloned();
        while let Some(tech) = current {
            out.push(tech.borrow().name.clone());
            current = tech.borrow().prerequisite.upgrade();
        }
        out.reverse();
        out
    }

    /// One line per tech, two spaces of indent per level, each line ending in '\n'.
    pub fn render(&self) -> String {
        let mut out = String::new();
        render_tech(&self.techs[&ROOT], 0, &mut out);
        out
    }

    /// Removes tech `id` and everything that depends on it. False for the start or an unknown id.
    pub fn remove(&mut self, id: u32) -> bool {
        if id == ROOT {
            return false;
        }
        let Some(tech) = self.techs.get(&id).cloned() else {
            return false;
        };
        detach(&tech);
        let mut doomed = Vec::new();
        collect(&tech, &mut doomed);
        self.techs
            .retain(|_, t| !doomed.iter().any(|d| Rc::ptr_eq(d, t)));
        true
    }

    /// Makes `new_prerequisite` the prerequisite of `id` (with its dependants), as its last unlock.
    /// Refuses (false) for the start, unknown ids, and moves under itself or its own dependants.
    pub fn reparent(&mut self, id: u32, new_prerequisite: u32) -> bool {
        if id == ROOT {
            return false;
        }
        let (Some(tech), Some(target)) = (self.techs.get(&id), self.techs.get(&new_prerequisite))
        else {
            return false;
        };
        let mut current = Some(Rc::clone(target));
        while let Some(t) = current {
            if Rc::ptr_eq(&t, tech) {
                return false;
            }
            current = t.borrow().prerequisite.upgrade();
        }
        detach(tech);
        tech.borrow_mut().prerequisite = Rc::downgrade(target);
        target.borrow_mut().unlocks.push(Rc::clone(tech));
        true
    }
}

fn render_tech(tech: &TechRef, depth: usize, out: &mut String) {
    let t = tech.borrow();
    out.push_str(&"  ".repeat(depth));
    out.push_str(&t.name);
    out.push('\n');
    for unlocked in &t.unlocks {
        render_tech(unlocked, depth + 1, out);
    }
}

fn collect(tech: &TechRef, out: &mut Vec<TechRef>) {
    out.push(Rc::clone(tech));
    for unlocked in &tech.borrow().unlocks {
        collect(unlocked, out);
    }
}

/// Removes `tech` from its prerequisite's unlocks.
fn detach(tech: &TechRef) {
    let prerequisite = tech.borrow().prerequisite.upgrade();
    if let Some(prerequisite) = prerequisite {
        prerequisite
            .borrow_mut()
            .unlocks
            .retain(|c| !Rc::ptr_eq(c, tech));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Automation
    ///   Logistics
    ///     Fast inserter
    ///   Electronics
    fn sample() -> (TechTree, u32, u32, u32) {
        let mut tree = TechTree::new("Automation");
        let logistics = tree.add(ROOT, "Logistics").unwrap();
        let fast_inserter = tree.add(logistics, "Fast inserter").unwrap();
        let electronics = tree.add(ROOT, "Electronics").unwrap();
        (tree, logistics, fast_inserter, electronics)
    }

    #[test]
    fn starting_tech_is_the_root() {
        let tree = TechTree::new("Automation");
        assert_eq!(tree.name(ROOT), Some(String::from("Automation")));
        assert_eq!(tree.name(99), None, "unknown id");
    }

    #[test]
    fn add_tech_returns_fresh_ids() {
        let (mut tree, logistics, fast_inserter, electronics) = sample();
        assert_eq!((logistics, fast_inserter, electronics), (1, 2, 3), "ids count up from 1");
        assert_eq!(tree.name(fast_inserter), Some(String::from("Fast inserter")));
        assert_eq!(tree.add(99, "x"), None, "unknown prerequisite");
    }

    #[test]
    fn path_lists_prerequisites() {
        let (tree, _, fast_inserter, _) = sample();
        assert_eq!(tree.path(fast_inserter), vec!["Automation", "Logistics", "Fast inserter"]);
        assert_eq!(tree.path(ROOT), vec!["Automation"]);
        assert!(tree.path(99).is_empty(), "unknown id has no path");
    }

    #[test]
    fn render_indents_dependants() {
        let (tree, _, _, _) = sample();
        assert_eq!(
            tree.render(),
            "Automation\n  Logistics\n    Fast inserter\n  Electronics\n"
        );
    }

    #[test]
    fn remove_deletes_dependants() {
        let (mut tree, logistics, fast_inserter, _) = sample();
        assert!(tree.remove(logistics));
        assert_eq!(tree.render(), "Automation\n  Electronics\n");
        assert_eq!(tree.name(logistics), None);
        assert_eq!(
            tree.name(fast_inserter),
            None,
            "a tech that depended on a removed tech goes too"
        );
        assert!(!tree.remove(ROOT), "the starting tech cannot be removed");
        assert!(!tree.remove(99), "unknown id");
    }

    #[test]
    fn reparent_moves_dependants() {
        let (mut tree, _, fast_inserter, electronics) = sample();
        assert!(tree.reparent(fast_inserter, electronics));
        assert_eq!(
            tree.render(),
            "Automation\n  Logistics\n  Electronics\n    Fast inserter\n"
        );
        assert_eq!(
            tree.path(fast_inserter),
            vec!["Automation", "Electronics", "Fast inserter"]
        );
    }

    #[test]
    fn reparent_refuses_cycles() {
        let (mut tree, logistics, fast_inserter, _) = sample();
        assert!(
            !tree.reparent(logistics, fast_inserter),
            "Fast inserter depends on Logistics: that would make a loop"
        );
        assert!(!tree.reparent(logistics, logistics), "a tech cannot depend on itself");
        assert!(!tree.reparent(ROOT, logistics), "the starting tech cannot move");
        assert!(!tree.reparent(logistics, 99), "unknown target");
        assert_eq!(
            tree.render(),
            "Automation\n  Logistics\n    Fast inserter\n  Electronics\n",
            "tree unchanged"
        );
    }
}
