An outliner (Workflowy, Notion, a file tree) is a tree you edit all the time: add a note, drag it somewhere else, delete a whole branch. Build its engine with everything in this world: `Rc` owns down the tree, `Weak` points up, and `RefCell` lets you change a node that others share.

**The design.** A `Node` owns its `children` as `NodeRef`s, the type alias from quest 11, and points up at its parent with a `Weak`. `Doc` keeps a `HashMap<u32, NodeRef>`, so any node can be found by its **id**, a plain number. The root always has id `0`, and `Doc::new` is written for you.

```rust
let mut doc = Doc::new("Title");
let a = doc.add(0, "groceries").unwrap();   // id 1
let b = doc.add(a, "milk").unwrap();        // id 2
println!("{}", doc.render());
// Title
//   groceries
//     milk
```

**The rules.** One method per objective, solved top to bottom:

- `text(id)` gives the node's text, or `None` for an unknown id.
- `add(parent, text)` gives `Some(new id)`, or `None` if `parent` does not exist. Ids count up from 1 and are never reused.
- `path(id)` lists the texts from the root down to the node. An unknown id gives an empty `Vec`.
- `render()` writes one line per node, two spaces of indent per level, each line ending in `'\n'`.
- `remove(id)` deletes the node and its whole subtree, map entries included, and gives `true`. The root and unknown ids give `false`.
- `move_node(id, new_parent)` re-attaches the node, with its subtree, as the last child of `new_parent`. It refuses (`false`) the root, unknown ids, and a `new_parent` that is the node itself or anything below it, because that would make a loop.

**You have built each part.** `path` is quest 15's breadcrumb. `render` is quest 12's walk with a depth passed down. Taking a node out of its parent is quest 16's `detach`. The loop check in `move_node` is quest 15's `has_ancestor`, comparing with `Rc::ptr_eq` instead of titles.

**One borrow per statement.** The panic to expect here is `already borrowed`: a `borrow()` still alive inside a long expression when the next step calls `borrow_mut()` on the same node. Put each read in its own `let`, as in quests 15 and 16, so it ends at the `;`.

**For the curious: ids instead of pointers.** Many Rust programs skip `Rc<RefCell>` for trees and graphs. They keep every node in one `Vec` or map, called an **arena**, and link nodes by index or id. Then there are no counts, no runtime borrow checks and no cycles that can leak; the price is that a stale id may name a removed node, so every lookup gives an `Option`. Your `Doc` is halfway there, since its map already finds every node by id; crates such as `slotmap` and `petgraph` go the rest of the way.

## Your turn

- Implement `text`, `add`, `path`, `render`, `remove` and `move_node` in that order. The first objective is a two-minute win.
- Each `// TODO:` names the helper calls that method needs.
- When the tests are green, press Run to edit a tree by hand.
