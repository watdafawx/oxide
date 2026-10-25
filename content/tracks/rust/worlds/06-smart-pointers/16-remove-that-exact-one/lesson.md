Deleting a note means taking it out of its parent's `children`. But two notes can look exactly alike, say two notes called "Milk", and only the one you deleted may go. So the question is "is this the same note?", not "does it look the same?".

**Same or equal.** `==` compares contents, so two separate notes with the same title would count as equal. `Rc::ptr_eq(&a, &b)` compares addresses instead: it is `true` only when both `Rc`s point at the very same value on the heap. Every `Rc::clone` of a note is `ptr_eq` to it; a look-alike never is.

**Keep the rest.** `v.retain(|x| keep)` from world 03 keeps the items for which the closure gives `true` and removes the rest. With `ptr_eq` inside, it removes exactly one note: keep every child that is *not* the same as this note.

```rust
let first = Rc::new(String::from("Milk"));
let again = Rc::clone(&first);
let other = Rc::new(String::from("Milk"));

println!("{}", first == other);             // true: same text
println!("{}", Rc::ptr_eq(&first, &again)); // true: the same value
println!("{}", Rc::ptr_eq(&first, &other)); // false: a look-alike
```

**Borrow the parent first.** To change the parent's `children` you need its `Rc`. Take it in its own statement, `let parent = note.borrow().parent.upgrade();`, so the borrow of the note ends at the `;`. Then `borrow_mut()` the parent; a note with no parent has nothing to detach from.

## Your turn

- In `is_same`, return whether `a` and `b` point at the same note.
- In `detach`, get the note's parent; if there is one, `retain` every child of it except this exact note.
