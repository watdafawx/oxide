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
        self.nodes.get(&id).map(|n| n.borrow().text.clone())
    }

    /// Adds a child under `parent`. Returns the new id, or None if `parent` does not exist.
    pub fn add(&mut self, parent: u32, text: &str) -> Option<u32> {
        let parent_node = self.nodes.get(&parent)?;
        let id = self.next_id;
        self.next_id += 1;
        let node = Rc::new(RefCell::new(Node {
            text: text.to_string(),
            children: Vec::new(),
            parent: Rc::downgrade(parent_node),
        }));
        parent_node.borrow_mut().children.push(Rc::clone(&node));
        self.nodes.insert(id, node);
        Some(id)
    }

    /// Texts from the root down to node `id`. Empty if `id` does not exist.
    pub fn path(&self, id: u32) -> Vec<String> {
        let mut out = Vec::new();
        let mut current = self.nodes.get(&id).cloned();
        while let Some(node) = current {
            out.push(node.borrow().text.clone());
            current = node.borrow().parent.upgrade();
        }
        out.reverse();
        out
    }

    /// One line per node, two spaces of indent per level, each line ending in '\n'.
    pub fn render(&self) -> String {
        let mut out = String::new();
        render_node(&self.nodes[&ROOT], 0, &mut out);
        out
    }

    /// Removes node `id` and its whole subtree. False for the root or an unknown id.
    pub fn remove(&mut self, id: u32) -> bool {
        if id == ROOT {
            return false;
        }
        let Some(node) = self.nodes.get(&id).cloned() else {
            return false;
        };
        detach(&node);
        let mut doomed = Vec::new();
        collect(&node, &mut doomed);
        self.nodes
            .retain(|_, n| !doomed.iter().any(|d| Rc::ptr_eq(d, n)));
        true
    }

    /// Moves `id` (with its subtree) to be the last child of `new_parent`.
    /// Refuses (false) for the root, unknown ids, and moves under itself or its own descendants.
    pub fn move_node(&mut self, id: u32, new_parent: u32) -> bool {
        if id == ROOT {
            return false;
        }
        let (Some(node), Some(target)) = (self.nodes.get(&id), self.nodes.get(&new_parent)) else {
            return false;
        };
        let mut current = Some(Rc::clone(target));
        while let Some(n) = current {
            if Rc::ptr_eq(&n, node) {
                return false;
            }
            current = n.borrow().parent.upgrade();
        }
        detach(node);
        node.borrow_mut().parent = Rc::downgrade(target);
        target.borrow_mut().children.push(Rc::clone(node));
        true
    }
}

fn render_node(node: &NodeRef, depth: usize, out: &mut String) {
    let n = node.borrow();
    out.push_str(&"  ".repeat(depth));
    out.push_str(&n.text);
    out.push('\n');
    for child in &n.children {
        render_node(child, depth + 1, out);
    }
}

fn collect(node: &NodeRef, out: &mut Vec<NodeRef>) {
    out.push(Rc::clone(node));
    for child in &node.borrow().children {
        collect(child, out);
    }
}

/// Removes `node` from its parent's children list.
fn detach(node: &NodeRef) {
    let parent = node.borrow().parent.upgrade();
    if let Some(parent) = parent {
        parent
            .borrow_mut()
            .children
            .retain(|c| !Rc::ptr_eq(c, node));
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
