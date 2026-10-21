The assembler's build queue grows when you order something and shrinks when a job finishes. The calculator needs both moves, and neither should leave a mess behind.

**Add to the end.** `table.insert(queue, "pipe")` puts `"pipe"` after the last job. `#queue` is now one bigger.

**Take one out.** `table.remove(queue, 2)` removes the job at index 2 and **hands it back**, so you can keep it in a name.

```lua
local queue = {"iron-gear-wheel", "pipe", "iron-stick"}
table.insert(queue, "copper-cable")    -- gear, pipe, stick, cable
local done = table.remove(queue, 2)    -- "pipe"; the queue is gear, stick, cable
```

**The gap closes.** After a `remove`, every job behind the removed one moves down one place. The queue stays packed with no gaps.

**Why not `= nil`?** `queue[2] = nil` also empties slot 2, but nothing moves down. That leaves a **hole**, and with a hole `#queue` can report the wrong count. Use `table.remove`.

## Your turn

- In `add_job`, add the job at the end of the queue.
- In `finish_job`, remove slot `i` and return what was there.
- Save. Both objectives turn green.
