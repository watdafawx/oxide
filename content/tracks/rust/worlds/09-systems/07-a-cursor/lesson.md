A reply is not one number: it is several fields, one after another. The checker reads the first, then the next, then the next. It needs to remember how far it got, and a reply that ends too early must give an error, not garbage or a crash.

**Read in order.** Reading field by field means keeping a bookmark. Each read takes some bytes from where the bookmark is and moves it forward. Doing that by hand in every function, with `bytes[3..5]` and `bytes[5..9]`, is where off-by-one bugs come from.

**A cursor.** So wrap it once: a **cursor** is a struct holding the slice and a position, `pos`, that only moves forward. It borrows the bytes, like the struct with a lifetime from world 02, so `Cursor<'a>` hands out slices of the original reply without copying anything.

**Check before you move.** `take(n)` first asks "are there `n` bytes left?". If not, it returns `Err(UnexpectedEof)` and leaves `pos` alone. Moving only on success means a failed read changes nothing, so the caller can report exactly where the reply broke.

**Everything on one method.** Once `take` is right, every other read is built on it and none of them can read past the end. `u8()` is "take one byte and give back that byte": `self.take(1)?` leaves early on an error, and `[0]` picks the byte.

```rust
// A method that skips bytes it doesn't need, with the same check.
pub fn skip(&mut self, n: usize) -> Result<(), ParseError> {
    if self.remaining() < n {
        return Err(ParseError::UnexpectedEof);
    }
    self.pos += n;
    Ok(())
}
```

## Your turn

- In `take`, check that `n` bytes are left, then cut them out with `&self.data[self.pos..self.pos + n]` and move `pos` past them.
- In `u8`, take one byte and give back the byte itself.
