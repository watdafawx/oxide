The bar ticks on real CPU and RAM. The rest of Kinetik's 44 stats come from many different corners of Windows, and not all of them are open to every app. Before writing sensors, it helps to know which doors are locked.

**Free for any app.** CPU usage, RAM, network traffic, disk space, uptime and the battery come from Windows' own counters. Any app may read them. In this world `sysinfo` reads the first four and a Windows function reads the battery.

**Free through a driver.** NVIDIA's driver ships `nvml.dll`, which gives GPU usage, temperature, clocks and power to any app. The `nvml-wrapper` crate loads it. AMD and Intel cards have no such DLL; Kinetik uses LibreHardwareMonitor for those, which this track leaves out.

**Locked behind administrator.** CPU temperature needs either the PawnIO driver plus administrator rights, or an ACPI thermal zone, which most laptops have and most desktops lack. Game FPS needs administrator for event tracing. Kinetik asks to restart as administrator for these.

**Never pretend.** A number the app may not read shows `--`, never `0`. Hover the stat and Kinetik says why: "needs administrator", "not on this PC". This world ends with a `Missing` reason for every gap.

**Cheap or skipped.** Each sensor costs a little CPU. Kinetik only reads stats that are switched on, and reads slow ones, like ping, without freezing the bar. Both rules are quests here.
