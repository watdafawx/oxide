Soon the bar reads real sensors every second. Real CPU usage is different on every run, so a test cannot say "the bar should show 42%". Tests need a sensor whose numbers are known in advance.

**One shape for all.** `Sensor` is a trait: a sensor says which stat it reads, and `read` gives the current value or `None`. The bar will hold a list of `Box<dyn Sensor>` and never care which kind is which.

**The stand-in.** `ScriptedSensor` plays a list of numbers: the first read gives the first, the next read the second. When the script runs out it gives `None`, like a sensor that stopped answering.

**Take, then step.** `get(i)` gives `Some(&value)` at a place in the list and `None` past the end, so no bounds check is needed; `.copied()` turns the `&f64` into an `f64`. Take the value at `next`, then move `next` on. Peeking at the second number of a script, whether or not there is one:

```rust
let second = self.script.get(1).copied();
```

**Real ones later.** `CpuSensor` and `RamSensor` arrive at the end of this world with the same two methods. The bar will not notice the swap.

## Your turn

- In `ScriptedSensor::read`, take the value at `self.next`, add 1 to `self.next`, and give back the value.
- Save.
