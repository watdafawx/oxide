The calculator should still work when the caller does not say how many batches: assume one. Lua lets you skip arguments, so you need a way to fill in the gap.

**Missing means nil.** Call a function with fewer arguments than it has parameters and the extra parameters are `nil`. Lua does not complain.

**Pick the other one.** `a or b` gives `a` when `a` counts as yes, and otherwise gives `b`. So `batches or 1` is "the batches, or 1 when there are none".

```lua
function M.eggs_total(batches)
  batches = batches or 1    -- no batches given: assume 1
  return batches * 3
end
```

**The trap.** `or` treats `false` as missing too: `false or true` is `true`. For a number that is fine. For a yes-or-no setting it throws away a real `false`.

**Ask for nil.** To keep `false`, ask `if value == nil then` and replace the value only then. That gives the default for a missing value and nothing else.

## Your turn

- In `batches_to_bake`, return `n`, or 1 when `n` is missing.
- In `option`, return `default` when `value` is `nil`, otherwise `value`.
- Check that `option(false, true)` stays `false`.
