`tp` and `spawnpoint` pass block positions around all the time. A `BlockPos` is just three `i32`s, so you would expect it to copy like a number. Use one twice and Rust says it moved. Your own types move by default; you have to say that they may be copied.

**Try it.** `spawn_and_home` gives back `(pos, pos)`: the same position as spawn and as home. Rust refuses, the same way it refused for a `String`:

```text
error[E0382]: use of moved value: `pos`
```

**Why it is not automatic.** Copying a struct's bytes is only safe when nothing in it points at the heap. Rust will not guess: if someone later added a `String` field, say a world name, silent copies would turn into double frees. So you promise it, on purpose, where everyone can see.

**A note to the compiler.** A line like `#[derive(Debug, PartialEq)]` above a struct is an **attribute**: an instruction to the compiler about the item below it. You have seen it on starters since the crafting calculator, and the `#[cfg(test)]` above every test module is one too.

**Derive writes code for you.** `derive` asks the compiler to write standard code for the type. `Debug` lets a failing test print the value, and `PartialEq` lets you compare two of them with `==`, which is what `assert_eq!` needs.

**Clone and Copy.** Add `Clone` and the type gets a `.clone()` method. Add `Copy` as well and every `let b = a;` and every call copies it silently, like a `u32`. Abilities like these are called **traits**; world 05 shows how to write your own.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct ChunkPos { x: i32, z: i32 }     // only numbers: may be Copy

#[derive(Debug, Clone, PartialEq)]
struct Ban { reason: String }          // owns heap text: Clone only
```

**The field rule.** `Copy` needs `Clone` beside it, and a struct may be `Copy` only when every field is. A `String` field, like a player's name, means `Clone` only: you call `.clone()` and pay for the copy where you can see it.

## Your turn

- Save and read the error under `spawn_and_home`.
- Add `Clone, Copy` to the derive line above `BlockPos`.
- Add `Clone` to the derive line above `Player`.
