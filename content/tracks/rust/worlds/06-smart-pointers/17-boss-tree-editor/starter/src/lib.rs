use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub const ROOT: u32 = 0;

/// A type alias (quest 11): a short name for a shared, changeable node.
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
        // TODO: look `id` up with self.nodes.get(&id) (an Option<&NodeRef>) and clone the node's text
        todo!()
    }

    /// Adds a child under `parent`. Returns the new id, or None if `parent` does not exist.
    pub fn add(&mut self, parent: u32, text: &str) -> Option<u32> {
        // TODO: find the parent node (None if unknown: `?` works on an Option);
        // give the new node self.next_id and bump it; it points up at its parent with a Weak,
        // the parent owns it in `children`; register it with self.nodes.insert(id, node)
        todo!()
    }

    /// Texts from the root down to node `id`. Empty if `id` does not exist.
    pub fn path(&self, id: u32) -> Vec<String> {
        // TODO: walk from the node up to the root with while let (quest 15), collecting texts;
        // self.nodes.get(&id).cloned() turns the Option<&NodeRef> into an Option<NodeRef> to start from
        todo!()
    }

    /// One line per node, two spaces of indent per level, each line ending in '\n'.
    pub fn render(&self) -> String {
        // TODO: write a recursive helper fn(node, depth, out: &mut String) over the root and its children
        // (quest 12). "  ".repeat(depth) is the indent, out.push_str(&text) appends text,
        // out.push('\n') ends the line. &self.nodes[&ROOT] is the root.
        todo!()
    }

    /// Removes node `id` and its whole subtree. False for the root or an unknown id.
    pub fn remove(&mut self, id: u32) -> bool {
        // TODO: refuse ROOT and unknown ids; detach the node from its parent (quest 16);
        // collect the node and everything below it into a Vec<NodeRef> (recursion), then
        // self.nodes.retain(|_, n| keep) with keep = "no collected node is ptr_eq to n"
        // (doomed.iter().any(|d| Rc::ptr_eq(d, n)) asks whether one is)
        todo!()
    }

    /// Moves `id` (with its subtree) to be the last child of `new_parent`.
    /// Refuses (false) for the root, unknown ids, and moves under itself or its own descendants.
    pub fn move_node(&mut self, id: u32, new_parent: u32) -> bool {
        // TODO: refuse ROOT; look up both nodes (false if either is unknown);
        // walk up from the target, starting at the target itself, and refuse if you meet `node`;
        // then detach it from its old parent, set its parent to a Weak of the target,
        // and push it onto the target's children
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
