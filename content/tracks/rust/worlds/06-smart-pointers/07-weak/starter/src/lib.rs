use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct TreeNode {
    pub name: String,
    pub parent: RefCell<Weak<TreeNode>>,
    pub children: RefCell<Vec<Rc<TreeNode>>>,
}

pub fn new(name: &str) -> Rc<TreeNode> {
    Rc::new(TreeNode {
        name: name.to_string(),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    })
}

/// Makes `child` a child of `parent`. The child's back-pointer must be Weak.
pub fn adopt(parent: &Rc<TreeNode>, child: &Rc<TreeNode>) {
    // TODO: the parent gets a (strong) child in its list; the child gets a weak link back to the parent
    todo!()
}

/// The parent's name, or None if there is no parent (or it was dropped).
pub fn parent_name(node: &TreeNode) -> Option<String> {
    // TODO: the parent's name, if the parent is still alive
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adopt_links_both_ways() {
        let root = new("root");
        let leaf = new("leaf");
        assert_eq!(parent_name(&leaf), None, "a fresh node has no parent");
        adopt(&root, &leaf);
        assert_eq!(root.children.borrow().len(), 1);
        assert_eq!(parent_name(&leaf), Some(String::from("root")));
    }

    #[test]
    fn parent_link_is_weak() {
        let root = new("root");
        let leaf = new("leaf");
        adopt(&root, &leaf);
        assert_eq!(Rc::strong_count(&root), 1, "child must not own its parent");
        assert_eq!(Rc::weak_count(&root), 1);
        assert_eq!(Rc::strong_count(&leaf), 2, "parent owns child: leaf + the Vec");
    }

    #[test]
    fn orphan_after_parent_dropped() {
        let leaf = new("leaf");
        {
            let root = new("root");
            adopt(&root, &leaf);
            assert_eq!(parent_name(&leaf), Some(String::from("root")));
        }
        assert_eq!(parent_name(&leaf), None, "root is gone, upgrade() must give None");
        assert_eq!(Rc::strong_count(&leaf), 1, "root's Vec released its handle");
    }
}
