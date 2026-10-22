Kinetik warns when the drive you choose runs low on space. A new stat, `DiskFree`, now sits in the model: label `C:`, written in bytes, and a warning below 10 GB. It needs a sensor.

**Every disk at once.** `Disks::new_with_refreshed_list()` lists every mounted drive. Each `Disk` has a `mount_point()`, like `C:\`, and `available_space()` in bytes.

**Find the one.** `list().iter().find(..)` gives the first disk the closure says yes to, as an `Option`. Compare the mount point with `Path::new("C:\\")`; the double backslash is one backslash inside a Rust string.

**Bail out early.** A PC with no C: drive is rare but possible. `?` after `find(..)` ends `read` with `None` right there, and the bar shows `--`:

```rust
let c = self.disks.list().iter().find(|d| d.mount_point() == Path::new("C:\\"))?;
```

**Refresh first.** `refresh(true)` updates the numbers, and drops disks that were unplugged since the last read.

## Your turn

- In `DiskSensor::read`, after the refresh, `find` drive C: with `?`.
- Give back `Some` of its `available_space()` as `f64`.
- Save.
