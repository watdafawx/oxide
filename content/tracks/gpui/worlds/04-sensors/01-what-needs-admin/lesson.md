The bar ticks on real CPU and RAM. The rest of Kinetik's 44 stats come from many different corners of Windows, and not all of them are open to every app. Before writing sensors, it helps to know which doors are locked.

**Counters Windows keeps.** Windows already counts a lot for itself: how busy each CPU core is, how much memory is in use, how many bytes each network adapter moved. These running totals are called **performance counters**, and Task Manager draws its graphs from them. Any app may read them, which is why they cost so little.

**Free for any app.** CPU usage, RAM, network traffic, disk space, uptime and the battery all come from those counters or from plain Windows functions. In this world the `sysinfo` crate reads the first four, and one Windows function reads the battery.

**Free through a driver.** A **driver** is the program that lets Windows talk to one piece of hardware. NVIDIA's driver also ships `nvml.dll`, a library that gives GPU usage, temperature, clocks and power to any app; the `nvml-wrapper` crate loads it. AMD and Intel cards have no such DLL. Kinetik uses LibreHardwareMonitor for those, which this track leaves out.

**Asking Windows questions.** Some numbers sit behind **WMI**, Windows Management Instrumentation: a built-in service you query with a small SQL-like language, such as "select the temperature from the thermal zones". It is slow to start and may answer "access denied", so it is the last place to look, not the first.

**Locked behind administrator.** CPU temperature needs either the PawnIO driver plus administrator rights, or an ACPI thermal zone through WMI, which most laptops have and most desktops lack. Game FPS needs administrator for event tracing. Kinetik asks to restart as administrator for these.

**Never pretend.** A number the app may not read shows `--`, never `0`. A `0` looks like a real reading; a `--` says "unknown". Hover the stat and Kinetik says why: "needs administrator", "not on this PC". This world ends with a reason for every gap.

**Cheap or skipped.** Each sensor costs a little CPU. Kinetik only reads stats that are switched on, and reads slow ones, like ping, without freezing the bar. Both rules are quests here.
