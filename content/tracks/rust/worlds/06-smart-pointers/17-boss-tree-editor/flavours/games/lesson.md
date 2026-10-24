A factory's tech tree gets rearranged all the time: add "Fast inserter" under Logistics, move Electronics under Automation, delete a whole branch. Build the engine of that editor with everything in this world: `Rc` owns down the tree, `Weak` points up, and `RefCell` lets you change a tech that others share.

**The design.** A `Tech` owns the techs it `unlocks` as `TechRef`s, the type alias from quest 11, and points up at its `prerequisite` with a `Weak`. `TechTree` keeps a `HashMap<u32, TechRef>`, so any tech can be found by its **id**, a plain number. The starting tech always has id `0`, and `TechTree::new` is written for you.

```rust
let mut tree = TechTree::new("Automation");
let a = tree.add(0, "Logistics").unwrap();       // id 1
let b = tree.add(a, "Fast inserter").unwrap();   // id 2
println!("{}", tree.render());
// Automation
//   Logistics
//     Fast inserter
```

**The rules.** One method per objective, solved top to bottom:

- `name(id)` gives the tech's name, or `None` for an unknown id.
- `add(prerequisite, name)` gives `Some(new id)`, or `None` if the prerequisite does not exist. Ids count up from 1 and are never reused.
- `path(id)` lists the names from the starting tech down to this one. An unknown id gives an empty `Vec`.
- `render()` writes one line per tech, two spaces of indent per level, each line ending in `'\n'`.
- `remove(id)` deletes the tech and everything that depends on it, map entries included, and gives `true`. The starting tech and unknown ids give `false`.
- `reparent(id, new_prerequisite)` re-attaches the tech, with its dependants, as the last unlock of `new_prerequisite`. It refuses (`false`) the starting tech, unknown ids, and a `new_prerequisite` that is the tech itself or one of its dependants, because that would make a loop.

**You have built each part.** `path` is quest 15's research path. `render` is quest 12's walk with a depth passed down. Taking a tech out of its prerequisite is quest 16's `detach`. The loop check in `reparent` is quest 15's `needs`, comparing with `Rc::ptr_eq` instead of names.

**One borrow per statement.** The panic to expect here is `already borrowed`: a `borrow()` still alive inside a long expression when the next step calls `borrow_mut()` on the same tech. Put each read in its own `let`, as in quests 15 and 16, so it ends at the `;`.

**For the curious: ids instead of pointers.** Many Rust programs skip `Rc<RefCell>` for trees and graphs. They keep every node in one `Vec` or map, called an **arena**, and link nodes by index or id. Then there are no counts, no runtime borrow checks and no cycles that can leak; the price is that a stale id may name a removed tech, so every lookup gives an `Option`. Your `TechTree` is halfway there, since its map already finds every tech by id; crates such as `slotmap` and `petgraph` go the rest of the way.

## Your turn

- Implement `name`, `add`, `path`, `render`, `remove` and `reparent` in that order. The first objective is a two-minute win.
- Each `// TODO:` names the helper calls that method needs.
- When the tests are green, press Run to edit the tech tree by hand.
