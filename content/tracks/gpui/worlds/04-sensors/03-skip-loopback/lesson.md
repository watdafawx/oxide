On the desktop this track was written on, sysinfo lists six adapters: Ethernet, a Hyper-V switch, a Teredo tunnel, ZeroTier, and two filter-driver copies of ZeroTier. Summing all of them counts the same download two or three times. The bar has to skip the ones that only mirror real traffic.

**Who to skip.** Loopback talks to itself. Teredo and isatap are tunnels whose traffic also crosses a real adapter. `vEthernet` is Hyper-V's virtual switch. Names with `Filter` are VPN and game-booster drivers that list a second copy of an adapter.

**A list of words.** `SKIP` holds those five words. An adapter counts unless its name contains one of them.

**Any of them.** `iter().any(..)` is true as soon as one entry matches, and stops looking there. Put `!` in front for "none of them":

```rust
!SKIP.iter().any(|skip| name.contains(skip))
```

**Used in the sum.** `NetSpeedSensor::read` already filters the adapter list with `counts_toward_total` before adding up. Fix the function and both speeds are right.

## Your turn

- In `counts_toward_total`, give back true unless `name` contains any word in `SKIP`.
- Save.
