`#[derive(ChunkInfo)]` works on `Chunk`. The indexer also has `Section<T: Clone>`, a slice of a chunk that stores blocks, light and palette as any type, and putting the derive on it stops the build:

```text
error[E0107]: missing generics for struct `Section`
   |     struct Section<T: Clone> {
   |            ^^^^^^^ expected 1 generic argument
```

**What the macro wrote.** Your `quote!` writes `impl Section { ... }`. But there is no type `Section`, only `Section<u8>`, `Section<&str>` and so on, one for each `T`. The error points at the struct because the bad code was written by your macro there; its "help" line is confused for the same reason.

**What it should write.** By hand you would write `impl<T: Clone> Section<T> { ... }`. The generics appear twice, differently: after `impl` they are declared with their bounds (`<T: Clone>`), after the type name they are only used (`<T>`). A `where` clause, if the type has one, goes before the `{`.

```rust
// by hand, for struct Labelled<T: Display> { value: T }
impl<T: Display> Labelled<T> {
//  ^^^^^^^^^^^^ declare T, with its bound
//                       ^^^ use T, names only
    fn label(&self) -> String {
        format!("[{}]", self.value)
    }
}
```

**Three pieces, ready to splice.** `input.generics.split_for_impl()` returns those three pieces as a tuple: the impl generics, the type generics and the where clause. Each is empty for a type with no generics, so the same `quote!` keeps working on `Chunk`, and on any type with any bounds.

## Your turn

- In `derive_chunk_info`, take the three pieces from `input.generics.split_for_impl()`.
- Splice them into the impl line: after `impl`, after `#name`, and before the `{`.
