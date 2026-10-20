use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub value: i32,
    pub children: Vec<Rc<RefCell<Node>>>,
}

/// A new node with no children.
pub fn new_node(value: i32) -> Rc<RefCell<Node>> {
    // TODO: a new Node with no children yet, wrapped so it can be shared and edited
    todo!()
}

/// Appends `child` to `parent`'s children.
pub fn add_child(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
    // TODO: add the child to the parent's children
    todo!()
}

/// The node's value plus the totals of all its descendants.
pub fn total(node: &Rc<RefCell<Node>>) -> i32 {
    // TODO: the node's own value plus the totals of its children (recursive)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_is_a_leaf() {
        let n = new_node(5);
        assert_eq!(n.borrow().value, 5);
        assert_eq!(n.borrow().children.len(), 0);
    }

    #[test]
    fn add_child_links_parent() {
        let root = new_node(1);
        add_child(&root, new_node(2));
        assert_eq!(root.borrow().children.len(), 1);
        assert_eq!(root.borrow().children[0].borrow().value, 2);
    }

    #[test]
    fn shared_child_changes_show_up() {
        let root = new_node(1);
        let child = new_node(2);
        add_child(&root, Rc::clone(&child));
        child.borrow_mut().value = 10;
        assert_eq!(total(&root), 11, "root should see the edit made through `child`");
    }

    #[test]
    fn total_sums_whole_tree() {
        let root = new_node(1);
        let a = new_node(2);
        let b = new_node(3);
        add_child(&a, new_node(4));
        add_child(&root, a);
        add_child(&root, b);
        assert_eq!(total(&root), 10);
    }
}
