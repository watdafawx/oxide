Every piece of mail is exactly one of three things: a letter, a parcel or freight, and the pallet size depends on which. A number or a piece of text could hold "letter", but it could also hold nonsense. You want a type that can only be one of the three.

**A closed list.** An **enum** is a type with a fixed list of choices. Each choice is a **variant**. You list them once, and nothing else is allowed:

```rust
enum Weather {
    Sun,
    Rain,
    Snow,
}

fn umbrellas(weather: Weather) -> u32 {
    match weather {
        Weather::Sun => 0,
        Weather::Rain => 2,
        Weather::Snow => 1,
    }
}
```

**Pick one.** You write a value of an enum with the type's name, two colons and the variant: `Weather::Rain`.

**Open it with match.** `match` looks at a value and runs the arm that fits. Each arm is a pattern, an arrow and a result, followed by a comma. Like `if`, the whole `match` is a value, so it can be a function's last line.

**No gaps allowed.** If a match forgets a variant, Rust refuses to build and names the one you missed. That is the whole point of an enum: add a choice later and the compiler shows you every place that must handle it.

```text
error[E0004]: non-exhaustive patterns: `Weather::Snow` not covered
```

## Your turn

- In `max_per_pallet`, match on `kind`: a letter holds 64 per pallet, a parcel 16, freight 1.
- Save. Both objectives turn green.
