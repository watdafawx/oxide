Opinions about GUI libraries are cheap, so this world measured them. A small task manager (every process, refreshed every second, sorted by CPU) was built in egui twice (two renderers), in GPUI and in Tauri, in release mode, and measured on one Windows 11 PC: a Ryzen 7 9800X3D with an RTX 5070 Ti, running about 330 processes.

**Two kinds of memory.** **Working set** is the memory a process has in RAM right now, shared pieces included. **Private memory** is memory set aside for that process alone, in RAM or not. They differ, so each number here says which one.

**Memory differs most.** By working set, GPUI used about 62 MB. egui used 80 to 140 MB with its OpenGL renderer and 240 MB with its default one. Tauri used about 245 MB private (430 MB working set), spread over seven processes: the app and six belonging to WebView2, because a web view is a browser.

**egui's renderer is a choice.** egui 0.37 draws through a library called wgpu unless told otherwise, and the default build set aside about 480 MB of private memory, most of it for the graphics driver. Switching egui to its `glow` (OpenGL) renderer, one line in `Cargo.toml`, dropped that to 75 to 130 MB private. Same egui code, same window.

**CPU at rest.** A program with no window that only reads the 330 processes once a second used about 8% of one core. egui with glow landed at about 6%, GPUI at about 10%: drawing a quiet list costs a few points at most. Tauri used about 17%. Its page rebuilt all 330 rows as HTML every second, not just the visible ones, which explains part of that.

**First window.** egui and Tauri showed a window in about a quarter of a second. GPUI took about three quarters, because it sets up DirectX and its text engine first. Tauri's page still has to load and draw its rows after that.

**Size and build.** The release exe was 7.5 MB for egui (glow), 11 MB for GPUI and 8.6 MB for Tauri, page included. The first release build took about 1.5 minutes for egui, 1.7 for Tauri and 2.3 for GPUI; after that every route rebuilds in seconds.

**What it means here.** The big gaps are the web view and egui's renderer, not egui against GPUI. egui with glow opens fastest and idles lowest; GPUI holds the least memory and opens half a second later. Either one beats the web route for a light tool, and CPU at rest is mostly how you read the processes, which is the next world's job.
