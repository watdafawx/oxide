use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub const ROOT: u32 = 0;

type NodeRef = Rc<RefCell<Node>>;

pub struct Node {
    pub text: String,
    pub children: Vec<NodeRef>,
    pub parent: Weak<RefCell<Node>>,
}

pub struct Doc {
    nodes: HashMap<u32, NodeRef>,
    next_id: u32,
}

impl Doc {
    /// A document with only the root node (id 0) holding `title`.
    pub fn new(title: &str) -> Doc {
        let root = Rc::new(RefCell::new(Node {
            text: title.to_string(),
            children: Vec::new(),
            parent: Weak::new(),
        }));
        let mut nodes = HashMap::new();
        nodes.insert(ROOT, root);
        Doc { nodes, next_id: 1 }
    }

    /// The text of node `id`, or None if there is no such node.
    pub fn text(&self, id: u32) -> Option<String> {
        // TODO: look `id` up in self.nodes and clone its text
        todo!()
    }

    /// Adds a child under `parent`. Returns the new id, or None if `parent` does not exist.
    pub fn add(&mut self, parent: u32, text: &str) -> Option<u32> {
        // TODO: find the parent node (None if unknown); give the new node the next id;
        // it must point up at its parent without owning it; the parent owns it in `children`;
        // register it in self.nodes
        todo!()
    }

    /// Texts from the root down to node `id`. Empty if `id` does not exist.
    pub fn path(&self, id: u32) -> Vec<String> {
        // TODO: walk from the node up to the root, collecting texts; reverse at the end
        todo!()
    }

    /// One line per node, two spaces of indent per level, each line ending in '\n'.
    pub fn render(&self) -> String {
        // TODO: write a recursive helper over the root node and its children
        todo!()
    }

    /// Removes node `id` and its whole subtree. False for the root or an unknown id.
    pub fn remove(&mut self, id: u32) -> bool {
        // TODO: refuse ROOT / unknown ids; detach from the parent's children;
        // collect the subtree and drop its entries from self.nodes
        todo!()
    }

    /// Moves `id` (with its subtree) to be the last child of `new_parent`.
    /// Refuses (false) for the root, unknown ids, and moves under itself or its own descendants.
    pub fn move_node(&mut self, id: u32, new_parent: u32) -> bool {
        // TODO: look up both nodes; walk up from the target and refuse if you meet `node`;
        // detach from the old parent, point at the new one, and become one of its children
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Title
    ///   a
    ///     b
    ///   c
    fn sample() -> (Doc, u32, u32, u32) {
        let mut doc = Doc::new("Title");
        let a = doc.add(ROOT, "a").unwrap();
        let b = doc.add(a, "b").unwrap();
        let c = doc.add(ROOT, "c").unwrap();
        (doc, a, b, c)
    }

    #[test]
    fn new_doc_has_root_title() {
        let doc = Doc::new("Title");
        assert_eq!(doc.text(ROOT), Some(String::from("Title")));
        assert_eq!(doc.text(99), None, "unknown id");
    }

    #[test]
    fn add_returns_fresh_ids() {
        let (mut doc, a, b, c) = sample();
        assert_eq!((a, b, c), (1, 2, 3), "ids count up from 1");
        assert_eq!(doc.text(b), Some(String::from("b")));
        assert_eq!(doc.add(99, "x"), None, "unknown parent");
    }

    #[test]
    fn path_lists_ancestors() {
        let (doc, _, b, _) = sample();
        assert_eq!(doc.path(b), vec!["Title", "a", "b"]);
        assert_eq!(doc.path(ROOT), vec!["Title"]);
        assert!(doc.path(99).is_empty(), "unknown id has no path");
    }

    #[test]
    fn render_indents_children() {
        let (doc, _, _, _) = sample();
        assert_eq!(doc.render(), "Title\n  a\n    b\n  c\n");
    }

    #[test]
    fn remove_deletes_subtree() {
        let (mut doc, a, b, _) = sample();
        assert!(doc.remove(a));
        assert_eq!(doc.render(), "Title\n  c\n");
        assert_eq!(doc.text(a), None);
        assert_eq!(doc.text(b), None, "the child of a removed node goes too");
        assert!(!doc.remove(ROOT), "the root cannot be removed");
        assert!(!doc.remove(99), "unknown id");
    }

    #[test]
    fn move_node_reparents() {
        let (mut doc, _, b, c) = sample();
        assert!(doc.move_node(b, c));
        assert_eq!(doc.render(), "Title\n  a\n  c\n    b\n");
        assert_eq!(doc.path(b), vec!["Title", "c", "b"]);
    }

    #[test]
    fn move_refuses_cycles() {
        let (mut doc, a, b, _) = sample();
        assert!(!doc.move_node(a, b), "b is a's child: that would make a loop");
        assert!(!doc.move_node(a, a), "a node cannot move under itself");
        assert!(!doc.move_node(ROOT, a), "the root cannot move");
        assert!(!doc.move_node(a, 99), "unknown target");
        assert_eq!(doc.render(), "Title\n  a\n    b\n  c\n", "tree unchanged");
    }
}
