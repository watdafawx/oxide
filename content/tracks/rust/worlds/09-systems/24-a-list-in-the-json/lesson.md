Some servers also send a short list of who is connected, `players.sample`, and the checker can show it. But a field can be missing, `null`, or the wrong shape entirely, and each case means something different. A `Value` tells you which, because it is an enum.

**A Value is an enum.** `serde_json::Value` has one variant per JSON shape: `Null`, `Bool`, `Number`, `String(String)`, `Array(Vec<Value>)` and `Object(..)`. So you can `match` on it like any enum from world 01, and the variant's data comes out in the pattern: `Value::Array(entries)` gives you the list.

**Match on a reference.** `v["players"]["sample"]` belongs to `v`, so match on a reference to it: `match &v["players"]["sample"] { ... }`. The parts then come out borrowed too, so `entries` is a `&Vec<Value>` you can loop over without copying anything.

**Missing is not wrong.** Many servers hide the list, so no `sample` at all, or `null`, both read as `Value::Null`: an empty list, not an error. A `sample` that is text or a number is a broken server, and that arm, `_`, returns an `Err`.

**Each entry.** Inside the list, each entry is another `Value`, an object here. `entry["name"].as_str()` from the last quest gives `Some(name)` or `None`, and `if let Some(name) = ...` (world 01) keeps the good ones.

```rust
match &v["version"]["name"] {
    Value::String(name) => println!("runs {name}"),
    Value::Null => println!("no version given"),
    other => println!("odd version field: {other}"),
}
```

## Your turn

- In `sample_names`, match on the shape of `players.sample`.
- For a list, collect the text `name` of each entry that has one; for `Null`, an empty list; anything else is the error in the doc comment.
