The hasher prints one line per jar: the hash, two spaces, the path. That is the layout `sha256sum` and similar tools use, so a modpack author can publish the list and players can check their `mods/` folder against it. Hashes are always shown in hex, and always with the same number of digits.

**Print in hex.** In a format string, `x` after the colon prints a number in hexadecimal: `format!("{:x}", 255)` is `"ff"`, and `{n:x}` does the same for a variable `n`. `X` gives capital letters; checksum tools use lowercase.

**Pad with zeros.** The collections world lined up columns with a width, `{:>8}`. Put a `0` in front of the width and the padding is zeros instead of spaces: `format!("{:08x}", 255)` is `"000000ff"`. A `u64` needs at most 16 hex digits, so `016x` gives every hash the same length, even one that happens to start with zeros.

```rust
let colour: u32 = 0x00ff_8800;
let text = format!("#{colour:06x}");    // "#ff8800"
let short = format!("{colour:x}");      // "ff8800"
```

## Your turn

- In `hex_line`, return the hash as 16 lowercase, zero-padded hex digits, then two spaces, then `path.display()`.
