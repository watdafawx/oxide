Each note shows a word count that includes every note under it. That is quest 02's walk again, with two differences: a note can have many children instead of one "rest", and each step goes through a `RefCell`.

**Same shape, more children.** The total is "my words plus the total of each child". The base case is a note with no children: the loop over the children runs zero times and you are left with its own words. No `Done` arm is needed.

**Borrow to read.** `note.borrow()` gives a reader. Keep it in a variable, `let n = note.borrow();`, then use `n.words` and loop `for child in &n.children`. Each child is itself a `NoteRef`, ready for the recursive call.

```rust
fn depth(note: &NoteRef) -> u32 {
    let n = note.borrow();
    let mut deepest = 0;
    for child in &n.children {
        let d = depth(child);
        if d > deepest {
            deepest = d;
        }
    }
    deepest + 1
}
```

**Readers are fine.** While `n` is alive the note is borrowed for reading, and the recursive calls borrow *other* notes, its children. Many readers at once are allowed, so nothing panics. A `borrow_mut()` of a note while `n` still reads it would panic (quest 10), which is why the habit is to keep borrows short.

## Your turn

- In `total_words`, start from the note's own words and add `total_words` of each child.
- `note_count` has the same shape: each note counts 1, plus the count of each child.
