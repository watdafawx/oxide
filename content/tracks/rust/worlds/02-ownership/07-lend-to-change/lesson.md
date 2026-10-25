Some functions have to change the caller's value: add a word, grow a list. A `&` borrow only lets a function look, so Rust has a second kind of borrow, one that may write.

**A borrow that writes.** `&mut word` lends `word` so the function may change it. The parameter type says the same: `s: &mut String`. Changes land in the caller's value, because there is only ever one value.

**Opt in twice.** The variable must be `let mut`, and the call must say `&mut`. Both are on purpose: reading a call like `polite(&mut word)` tells you `word` may change, without opening `polite`.

**Like &mut self.** This is the `&mut self` from "Change it in place" in the last world, for any parameter.

```rust
fn polite(s: &mut String) {
    s.push_str(", please");   // push_str adds text to the end
}

let mut word = String::from("sit down");
polite(&mut word);
// word is now "sit down, please"
```

**Lists too.** A `&mut Vec<i32>` works the same: `v.push(7)` adds to the caller's list, and `v.pop()` takes the last item off and gives it back as an `Option`, `None` when the list was empty.

## Your turn

- In `add_exclaim`, add `"!!"` to the end of `s`.
- In `push_twice`, push `x` onto `v` two times.
