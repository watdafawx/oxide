**Why now:** a factory's tech tree gets rearranged constantly: add "Fast inserter" under Logistics, move Electronics under Automation, delete a branch. Time to build the engine behind that editor with everything in this world: `Rc`, `RefCell`, `Weak`.

The design:

- A `Tech` owns the techs it unlocks (`unlocks: Vec<Rc<RefCell<Tech>>>`) and points up at its prerequisite with a **`Weak`**.
- `TechTree` keeps a `HashMap<u32, Rc<RefCell<Tech>>>` so any tech is found by its **id**. The starting tech always has id `0`.

```rust
let mut tree = TechTree::new("Automation");
let a = tree.add(0, "Logistics").unwrap();       // id 1
let b = tree.add(a, "Fast inserter").unwrap();   // id 2
println!("{}", tree.render());
// Automation
//   Logistics
//     Fast inserter
```

The rules:

- `add(prerequisite, name)` -> `Some(new id)`, or `None` if the prerequisite does not exist. Ids are never reused.
- `remove(id)` -> `true` if removed. The starting tech and unknown ids return `false`. The whole subtree goes, including its map entries.
- `reparent(id, new_prerequisite)` -> `true` if moved (appended as the last unlock). Refuse (`false`) for the starting tech, unknown ids, and when `new_prerequisite` is `id` itself or any of its dependants: that would be a cycle.
- `path(id)` -> names from the starting tech down to this tech. Unknown id: empty Vec.
- `render()` -> one line per tech, two spaces per depth level, each line ends in `\n`.

## Three things the boss needs

- **`Rc::ptr_eq(a, b)`** finds the same shared tech in a Vec: `unlocks.retain(|c| !Rc::ptr_eq(c, &tech))` drops exactly that child.
- **End a `borrow()` before you `borrow_mut()` the same cell.** Take the parent in its own statement, `let parent = tech.borrow().prerequisite.upgrade();`, so the `Ref` dies at the semicolon. Keep it alive in a bigger expression and the next `borrow_mut()` panics.
- **`let ... else`** unwraps or leaves early: `let Some(tech) = self.techs.get(&id) else { return false; };`.

- **More small tools:** `while let Some(x) = current { ...; current = next; }` loops while an `Option` is `Some`. `opt.cloned()` turns `Option<&Rc<T>>` into `Option<Rc<T>>`. `"  ".repeat(3)` is six spaces. `out.push_str(&text)` appends a `&str` to a `String`, `out.push('\n')` one char. `v.reverse()` flips a `Vec` in place. `map.retain(|id, value| keep)` drops the entries where the closure is `false`. `&map[&key]` reads a `HashMap` entry (panics when absent). `v.iter().any(|x| ...)` asks whether some item matches.

## Your turn

In `src/lib.rs` implement `name`, `add`, `path`, `render`, `remove` and `reparent`. `TechTree::new` is done. The first objective is a two-minute win: start there.

Press Run to play it.
