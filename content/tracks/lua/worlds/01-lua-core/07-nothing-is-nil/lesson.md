A shelf in the pantry has slots, and a slot can hold an item or hold nothing. The calculator must tell "nothing" from "something", and Lua has a word for nothing.

**Nothing has a name.** `nil` is Lua's word for "no value". A function called without an argument gets `nil` for that parameter, with no error.

**Ask for nothing.** You can ask for it like any other value: `item == nil` is `true` when there is nothing there.

**Two ways to say no.** In an `if`, only `nil` and `false` count as no. Everything else counts as yes, including `0` and the empty text `""`. A slot holding a count of 0 is still holding something.

**Flip it.** `not` turns yes into no and no into yes: `not nil` is `true`, and `not 0` is `false`.

```lua
local slot = nil
local empty = slot == nil   -- true
local flipped = not nil     -- true
local real = not 0          -- false: 0 is a real value
```

## Your turn

- In `slot_is_empty`, return whether `item` is `nil`.
- In `is_falsy`, return the flipped `value` with `not`.
- Save. Both objectives turn green.
