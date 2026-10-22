Many PCs have two graphics chips: the one built into the processor and a real card. Kinetik's Stats page has a Graphics card setting, and its default, Auto, has to guess which one you care about. Its rule: the NVIDIA card if there is one, otherwise the card with the most video memory.

**Cards as data.** `GpuInfo` holds a card's name, `Vendor` and video memory. `pick_gpu` takes the list and gives back the index of the card to show, or `None` for an empty list.

**NVIDIA first.** `position(..)` gives the index of the first item the closure says yes to. If there is an NVIDIA card, that index is the answer, and the function can return early.

**Else the biggest.** `enumerate()` pairs each card with its index; `max_by_key(..)` keeps the pair with the most `vram_bytes`; `map` keeps only the index. An empty list gives `None` all the way through:

```rust
gpus.iter().enumerate().max_by_key(|(_, g)| g.vram_bytes).map(|(i, _)| i)
```

**Why VRAM.** The built-in chip borrows a little system memory; a real card has gigabytes of its own. More VRAM is a good sign of "the card games run on".

## Your turn

- In `pick_gpu`, return the first NVIDIA card's index with `position`.
- Otherwise give back the index with the most `vram_bytes`.
- Save.
