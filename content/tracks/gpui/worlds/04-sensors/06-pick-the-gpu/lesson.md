Many PCs have two graphics chips: the one built into the processor and a real card. Kinetik's Stats page has a Graphics card setting, and its default, Auto, has to guess which one you care about. Its rule: the NVIDIA card if there is one, otherwise the card with the most video memory.

**Cards as data.** `GpuInfo` holds a card's name, `Vendor` and video memory (VRAM). `pick_gpu` takes the list and gives back the index of the card to show, or `None` for an empty list.

**NVIDIA first.** `position(..)` gives the index of the first item the closure says yes to, as an `Option<usize>`. If there is an NVIDIA card, that index is the answer, and `if let Some(i) = ..` can `return` it early.

**Else the biggest.** Three steps in a chain. `enumerate()` pairs each item with its index, as `(index, item)`. `max_by_key(..)` keeps the pair whose key is largest. `map` then keeps only the index. An empty list gives `None` all the way through. The same chain, picking the fastest network adapter:

```rust
adapters.iter().enumerate().max_by_key(|(_, a)| a.speed_mbps).map(|(i, _)| i)
```

**Why VRAM.** The built-in chip borrows a little system memory; a real card has gigabytes of its own. More VRAM is a good sign of "the card games run on".

## Your turn

- In `pick_gpu`, return the first NVIDIA card's index early, if there is one.
- Otherwise give back the index of the card with the most video memory.
- Save.
