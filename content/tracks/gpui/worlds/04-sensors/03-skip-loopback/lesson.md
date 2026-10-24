On the desktop this track was written on, sysinfo lists six adapters: Ethernet, a Hyper-V switch, a Teredo tunnel, ZeroTier, and two filter-driver copies of ZeroTier. Summing all of them counts the same download two or three times. The bar has to skip the ones that only mirror real traffic.

**Who to skip.** Loopback is the PC talking to itself. Teredo and isatap are tunnels whose traffic also crosses a real adapter. `vEthernet` is Hyper-V's virtual switch. Names with `Filter` are VPN and game-booster drivers that list a second copy of an adapter.

**A list of words.** `SKIP` holds those five words. An adapter counts unless its name contains one of them. Your PC may list fewer adapters, or different ones; the rule is the same.

**Any of them.** `iter().any(..)` runs the closure on each entry and is true as soon as one says yes, and stops looking there. `!` in front turns it into "none of them". Here it checks a log line for any warning word:

```rust
let alarming = WORDS.iter().any(|word| line.contains(word));
```

**Used in the sum.** `NetSpeedSensor::read` already filters the adapter list with `counts_toward_total` before adding up. Fix the function and both speeds are right.

## Your turn

- In `counts_toward_total`, give back true unless `name` contains one of the words in `SKIP`.
- Save.
