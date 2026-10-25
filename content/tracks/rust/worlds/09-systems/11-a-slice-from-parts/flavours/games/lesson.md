Still in the aside, one level under the cursor. Your `PacketCursor` held a `&[u8]`, and you never asked what a slice is inside. It is two numbers, and taking it apart and putting it back together shows exactly what a safe wrapper promises. Here the slice is the player counts of a few servers.

**A slice is two numbers.** A `&[i32]` is the address of its first item plus how many items follow. `counts.as_ptr()` gives the address as a `*const i32` and `counts.len()` the count. That is all a slice is; the borrow checker adds the promise that both are right.

**Back from the parts.** `std::slice::from_raw_parts(ptr, len)` turns an address and a length back into a slice. It is `unsafe`, because Rust has to trust you that `ptr` is not null, is aligned, and really has `len` live items behind it that nobody changes meanwhile. This is what code receiving a buffer from the operating system does once, at the border.

**Hide it behind a safe function.** A slice's own `as_ptr()` and `len()` always meet those rules. So a function that takes a `&[i32]` and passes its parts to the raw version can never be misused, and it can be a normal, safe `fn`. The `unsafe` stays two lines long, and every caller gets a function that can't go wrong.

```rust
let ports = [25565u16, 25566, 19132];
// SAFETY: the pointer and the length 2 stay inside `ports`.
let java_ports = unsafe { std::slice::from_raw_parts(ports.as_ptr(), 2) };
assert_eq!(java_ports, &[25565, 25566]);
```

**Back to the pinger.** You won't write this again in this world. `TcpStream`, right after the two C quests, does it for you: it passes your buffer's parts to the operating system and gives back a safe `read`.

## Your turn

- In `total_players_raw`, rebuild the slice from `ptr` and `len` and add the counts up.
- In `total_players`, call `total_players_raw` with the slice's own pointer and length.
- Write a `// SAFETY:` comment above each `unsafe` block saying why it holds.
