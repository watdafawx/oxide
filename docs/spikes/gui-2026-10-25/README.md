# GUI spike, 2026-10-25

The measurements behind `taskman/00-pick-a-gui/10-the-numbers` and `docs/tracks/taskman.md`
section 2. Five tiny apps doing the same job (read every process with sysinfo once a second,
sort by CPU, show it), plus the script that measured them. `results.txt` is the raw output.

Build each with one shared, temporary target dir, and delete it afterwards (about 5 GB):

```
$env:CARGO_TARGET_DIR = "$env:TEMP\gui-spike-target"
cargo build --release --manifest-path docs/spikes/gui-2026-10-25/gpui_tm/Cargo.toml
```

`tauri_tm` needs icons first: copy `src-tauri/icons/icon.ico`, `32x32.png` and `128x128.png`
into `tauri_tm/icons/`.

Measure (window apps and the baseline alike):

```
pwsh -File docs/spikes/gui-2026-10-25/measure.ps1 -Exe "$env:CARGO_TARGET_DIR\release\gpui_tm.exe" -Settle 15 -Window 30
```

Measured on FOX: Ryzen 7 9800X3D, RTX 5070 Ti, Windows 11, rustc 1.98.1 MSVC.
