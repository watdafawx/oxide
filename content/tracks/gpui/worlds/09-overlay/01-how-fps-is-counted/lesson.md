Kinetik's overlay shows a game's FPS, its 1% lows, a frame-time graph and the base FPS before frame generation. It does all of that without injecting anything into the game. This world builds the same, starting from where the numbers come from.

**Frames are presents.** When a game finishes a frame it calls `Present`: "show this now". Count presents per second and you have FPS. The time between two presents is the frame time: 8.3 ms at 120 FPS, 16.7 ms at 60.

**Windows already tells.** Windows' graphics stack reports every present through event tracing (ETW), a built-in system for apps and drivers to log events. PresentMon, CapFrameX and Kinetik all listen there. No hooking, so anti-cheat has nothing to object to.

**It needs administrator.** Listening to another program's graphics events is privileged. Without admin, Kinetik shows `--` for FPS and says why. This world tests its maths on a recorded capture, and the live listener says "needs administrator" when it cannot run.

**Frame generation.** DLSS and FSR frame generation insert made-up frames between the real ones. The screen shows 120; the game drew 60. FPS counts what reaches the screen; base FPS counts only the game's own frames.

**Lows and stutter.** Average FPS hides hitches. The 1% low is the speed of the slowest 1% of frames; a frame-time graph shows each hitch as a spike.
