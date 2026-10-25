Deleting a tech means taking it out of its prerequisite's `unlocks`. But a modded tree can hold two techs that look exactly alike, say two called "Electronics", and only the one you deleted may go. So the question is "is this the same tech?", not "does it look the same?".

**Same or equal.** `==` compares contents, so two separate techs with the same name would count as equal. `Rc::ptr_eq(&a, &b)` compares addresses instead: it is `true` only when both `Rc`s point at the very same value on the heap. Every `Rc::clone` of a tech is `ptr_eq` to it; a look-alike never is.

**Keep the rest.** `v.retain(|x| keep)` from world 03 keeps the items for which the closure gives `true` and removes the rest. With `ptr_eq` inside, it removes exactly one tech: keep every unlock that is *not* the same as this tech.

```rust
let first = Rc::new(String::from("Electronics"));
let again = Rc::clone(&first);
let other = Rc::new(String::from("Electronics"));

println!("{}", first == other);             // true: same name
println!("{}", Rc::ptr_eq(&first, &again)); // true: the same value
println!("{}", Rc::ptr_eq(&first, &other)); // false: a look-alike
```

**Borrow the prerequisite first.** To change the prerequisite's `unlocks` you need its `Rc`. Take it in its own statement, `let pre = tech.borrow().prerequisite.upgrade();`, so the borrow of the tech ends at the `;`. Then `borrow_mut()` the prerequisite; the starting tech has nothing to detach from.

## Your turn

- In `is_same`, return whether `a` and `b` point at the same tech.
- In `detach`, get the tech's prerequisite; if there is one, `retain` every unlock of it except this exact tech.
