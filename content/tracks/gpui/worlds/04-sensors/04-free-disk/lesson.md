Kinetik warns when the drive you choose runs low on space. A new stat, `DiskFree`, now sits in the model: label `C:`, written in bytes, and a warning below 10 GB. It needs a sensor.

**Every disk at once.** `Disks::new_with_refreshed_list()` lists every mounted drive. Each `Disk` has a `mount_point()`, the path Windows reaches it by, like `C:\`, and `available_space()` in bytes.

**Find the one.** `list().iter().find(..)` gives the first disk the closure says yes to, as an `Option`. A mount point is a `Path`, not text, so compare it with a `Path`: `Path::new("C:\\")`. The double backslash is one backslash inside a Rust string.

**Bail out early.** A PC with no C: drive is rare but possible. `?` after `find(..)` ends the function with `None` right there, and the bar shows `--`. The same move, finding the Ethernet adapter by name:

```rust
let ethernet = adapters.iter().find(|a| a.name == "Ethernet")?;
Some(ethernet.speed_mbps as f64)
```

**Refresh first.** `refresh(true)` updates the numbers, and drops disks that were unplugged since the last read. `read` already does it before your line.

## Your turn

- In `DiskSensor::read`, after the refresh, `find` the disk whose mount point is `C:\`, and end with `None` when there is none.
- Give back `Some` of its free space as `f64`.
- Save.
