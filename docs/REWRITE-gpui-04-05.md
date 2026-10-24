# GPUI worlds 04-sensors and 05-taskbar: rewrite map

Both worlds were reworked in the golden shape (`docs/CONTENT.md`), following the audit
`docs/audit-2026-10-24/gpui-00-05.md` (04 scored 7, 05 scored 6.5). GPUI quests do not build on the
Linux sandbox, so the work is mostly in lessons, hints, points, labels and TODO comments; code changes
are small and mechanical (listed below). Ledger rows for the parent to merge:
`scratchpad/ledger/gpui-04-05.md`.

## World 04: 14 quests became 17

| old | new |
|---|---|
| 01-what-needs-admin (read) | 01-what-needs-admin: adds cards for performance counters, drivers and WMI, and why `--` beats `0` |
| 02-network-speed | 02-network-speed: `Instant`/`Duration` glossed, why `as_secs_f64`, example on songs per minute |
| 03-skip-loopback | 03-skip-loopback: example on log words, not the answer line |
| 04-free-disk | 04-free-disk: why compare with a `Path`; example finds an adapter |
| 05-gpu-via-nvml | 05-gpu-via-nvml: what a DLL is, why `.ok()` before `?`; the example is the given `temperature` |
| 06-pick-the-gpu | 06-pick-the-gpu: the chain explained step by step; example picks the fastest adapter |
| 07-cpu-temp-fallback | 07-cpu-temp-fallback: what WMI and ACPI are; example converts the other way |
| 08-battery (unsafe call + bit flags, 7 min) | 08-battery (the `unsafe` call, why, `// SAFETY:`; example `GetCursorPos`), **09-battery-flags** (new: bits, bit masks, "the bit worth 8", `&`). "bit 8 / bit 128" fixed everywhere, including the starter doc comment |
| 09-ping | 10-ping |
| 10-uptime | 11-uptime: `/` and `%` on their own cards; example splits minutes and seconds |
| 11-lazy-sensors | 12-lazy-sensors: example skips dividers with `continue` |
| (new) | **13-keep-the-ui-thread-free** (read): the UI thread, waiting vs blocking, foreground vs background executor, the hop (`read_with` → background `spawn` → `.await` → `this.update`), let-else, `Arc<Mutex>` and `Send` refreshers (rust 07), poisoned locks |
| 12-off-the-ui-thread | 14-off-the-ui-thread: still the `: Send` compile error, now with why the compiler cannot tell for `dyn Sensor`, supertraits, and why `cx.spawn` needs no Send |
| (new) | **15-write-the-hop**: the learner writes the background spawn, the `.await` and `this.update` + `break` in `start` |
| 13-why-its-missing | 16-why-its-missing: HRESULT card (top bit, hex, why `as i32`) |
| 14-boss-sensor-hub | 17-boss-sensor-hub: the code block is now a skeleton of `apply`; cards for the nested match on two Options, `insert`/`remove` return values, `*why`; lock poisoning now taught in 13 |

## World 05: 11 quests became 12

| old | new |
|---|---|
| 01-window-options | 01-window-options: `..Default::default()` and focus explained; example shows a normal window's values |
| 02-see-through | 02-see-through: example is the `Opaque` default |
| 03-find-the-taskbar | 03-find-the-taskbar: cards for windows/classes, HWND (a handle), UTF-16 and `w!`, why `unsafe`; app bars (`SHAppBarMessage`) for the curious; example finds `Progman` |
| 04-dock-maths | 04-dock-maths: screen coordinates card; example intersects a window with its monitor |
| 05-dpi | 05-dpi: why screens scale, DPI, `as f32` before dividing; example converts back to physical |
| 06-stay-on-top (4 ideas) | **06-a-seam-for-tests** (new read: seam, `WinApi`, `RealWin`, `FakeWin` as a spy, `Docker<A: WinApi>`), 07-stay-on-top (z-order, topmost, the HWND via `raw-window-handle`) |
| 07-light-or-dark | 08-light-or-dark: `render`'s `window` parameter named, exhaustive match; example groups two stats |
| 08-hide-in-fullscreen | 09-hide-in-fullscreen: "no fullscreen flag" card; example is the opposite check |
| 09-every-taskbar | 10-every-taskbar: why `copied()`. The lesson claimed "the boss sets `display_id`"; no code does, so it now says the boss keeps `all_taskbars` off and a second bar would need `display_id` |
| 10-taskbar-moved | 11-taskbar-moved: why the comparison works (`PartialEq`) |
| 11-boss-docked-bar | 12-boss-docked-bar: names `redock` (quest 11) and `bar_visible` (quest 09) as the parts folded into `tick`; code block is a skeleton; `taskkill` named. The `main.rs` docker loop already breaks when the bar is gone; kept |

## Code changes (not compiled: gpui does not build here)

- New `04-sensors/09-battery-flags`: copied from `08-battery/solution`, package `kinetik_04_battery_flags`; the starter's `decode_battery` is a `todo!`. `08-battery/starter` now has `decode_battery` written (from the solution), so 08 has one objective.
- New `04-sensors/15-write-the-hop`: copied from `14-off-the-ui-thread/solution`, package `kinetik_04_write_the_hop`, test renamed `bar_shows_reads_from_the_background`; the starter's `start` loop stops after `read_with` (TODOs in words).
- `14-off-the-ui-thread/starter/src/sensor.rs`: a TODO comment above the trait.
- TODO texts reworded to say the goal in words (04/04, 05, 07, 08, 09, 10; 05/03, 04, 11). No types, signatures or file layout changed.

## Ids and references

- Renumbered: 04 `09-ping`→`10`, `10-uptime`→`11`, `11-lazy-sensors`→`12`, `12-off-the-ui-thread`→`14`, `13-why-its-missing`→`16`, `14-boss-sensor-hub`→`17`; 05 `06-stay-on-top`→`07`, `07`→`08`, `08`→`09`, `09`→`10`, `10`→`11`, `11-boss-docked-bar`→`12`. Boss slugs kept; Cargo package names of moved quests kept.
- `05-taskbar/01-window-options` requires `04-sensors/17-boss-sensor-hub`.
- `06-interaction/01-what-a-listener-is` requires `05-taskbar/12-boss-docked-bar` (updated).
- `04-sensors/01-what-needs-admin` requires `03-live/16-boss-live-bar` (the 03 rewrite renumbered its boss); the rust requirement is unchanged.
- `docs/tracks/gpui.md` (the plan's quest tables for 04 and 05) still lists the old quests; not edited.
