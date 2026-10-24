`op` and `rename` change the server's data: a name gets a tag, the command queue grows. A `&` borrow only lets a function look, so Rust has a second kind of borrow, one that may write.

**A borrow that writes.** `&mut player` lends `player` so the function may change it. The parameter type says the same: `name: &mut String`. Changes land in the caller's value, because there is only ever one value.

**Opt in twice.** The variable must be `let mut`, and the call must say `&mut`. Both are on purpose: reading a call like `rename(&mut player)` tells you `player` may change, without opening `rename`.

**Like &mut self.** This is the `&mut self` from "Change it in place" in the crafting calculator, for any parameter.

```rust
fn rename(name: &mut String) {
    name.push_str("_afk");    // push_str adds text to the end
}

let mut player = String::from("Steve");
rename(&mut player);
// player is now "Steve_afk"
```

**Lists too.** A `&mut Vec<String>` works the same: `list.push(x)` adds to the caller's list, and `list.pop()` takes the last item off and gives it back as an `Option`, `None` when the list was empty.

## Your turn

- In `tag_as_op`, add `" [OP]"` to the end of `name`.
- In `queue_twice`, push `command` onto `queue` two times, as owned Strings.
