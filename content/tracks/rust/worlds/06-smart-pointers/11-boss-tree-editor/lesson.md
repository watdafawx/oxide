An outliner (Workflowy, Notion, a file tree) is a tree you edit constantly: add a note, drag it elsewhere, delete a branch. Time to build the engine with everything in this world.

The design:

- A `Node` owns its `children` (`Rc<RefCell<Node>>`) and points up with a **`Weak`** parent.
- `Doc` keeps a `HashMap<u32, Rc<RefCell<Node>>>` so any node is found by its **id**. The root always has id `0`.

```rust
let mut doc = Doc::new("Title");
let a = doc.add(0, "groceries").unwrap();   // id 1
let b = doc.add(a, "milk").unwrap();        // id 2
println!("{}", doc.render());
// Title
//   groceries
//     milk
```

The rules:

- `add(parent, text)` -> `Some(new id)`, or `None` if `parent` does not exist. Ids are never reused.
- `remove(id)` -> `true` if removed. The root and unknown ids return `false`. The whole subtree goes, including its map entries.
- `move_node(id, new_parent)` -> `true` if moved (appended as the last child). Refuse (`false`) for the root, unknown ids, and when `new_parent` is `id` itself or any of its descendants.
- `path(id)` -> texts from the root down to the node. Unknown id: empty Vec.
- `render()` -> one line per node, two spaces per depth level, each line ends in `\n`.

## Three things the boss needs

- **`Rc::ptr_eq(a, b)`** finds the same shared node in a Vec: `children.retain(|c| !Rc::ptr_eq(c, &node))` drops exactly that child.
- **End a `borrow()` before you `borrow_mut()` the same cell.** Take the parent in its own statement, `let parent = node.borrow().parent.upgrade();`, so the `Ref` dies at the semicolon. Keep it alive in a bigger expression and the next `borrow_mut()` panics.
- **`let ... else`** unwraps or leaves early: `let Some(node) = self.nodes.get(&id) else { return false; };`.

- **More small tools:** `while let Some(x) = current { ...; current = next; }` loops while an `Option` is `Some`. `opt.cloned()` turns `Option<&Rc<T>>` into `Option<Rc<T>>`. `"  ".repeat(3)` is six spaces. `out.push_str(&text)` appends a `&str` to a `String`, `out.push('\n')` one char. `v.reverse()` flips a `Vec` in place. `map.retain(|id, value| keep)` drops the entries where the closure is `false`. `&map[&key]` reads a `HashMap` entry (panics when absent). `v.iter().any(|x| ...)` asks whether some item matches.

## Your turn

In `src/lib.rs` implement `text`, `add`, `path`, `render`, `remove` and `move_node`. `Doc::new` is done. The first objective is a two-minute win: start there.

Press Run to play it.
