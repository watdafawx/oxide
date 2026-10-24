From here on the bar reacts to Windows: it raises itself when the taskbar comes to the front, moves when the tray grows, hides for a fullscreen game. A test cannot click a real taskbar or start a real game, and a test that moves real windows would fight with whatever you are doing. So before any of that, the Windows calls go behind a trait.

**A seam.** A **seam** is a place where you can swap one piece for another without changing the code around it. From the next quest on, the seam is a trait in `dock.rs`, `WinApi`. It lists the few things the bar needs from Windows: the taskbars, the window in front, make a window topmost, move one, show or hide one.

**The real one.** `RealWin` in `win.rs` implements `WinApi` with the real `unsafe` Win32 calls. `win.rs` is the only file that talks to Windows about windows, so it is also the only file to check when something goes wrong there.

**The fake one.** `FakeWin` implements the same trait with no Windows at all. A test sets its `taskbars` and `foreground` to whatever situation it wants, a tray that grew or a fullscreen game. Every action it is asked for, it records in `calls` as a `Call`, like `Call::Topmost(7)`. The test then asserts on that list: "was the bar raised, exactly once?".

**One Docker, either Windows.** The `Docker` that keeps a bar on the taskbar is written as `Docker<A: WinApi>`: a struct generic over any type that implements the trait, held in its `api` field. The app builds `Docker::new(RealWin, ..)` (the boss's `main.rs` does); the tests build `Docker::new(FakeWin::default(), ..)`. The Docker's code is the same in both, so what the tests check is what runs.

**Why not a trait object.** `Box<dyn WinApi>` would work too. The generic version lets a test read `docker.api.calls` directly, because the compiler knows the field is a `FakeWin`, and it costs nothing at run time.

**For the curious.** This is the same move as `trait Sensor` and `ScriptedSensor` in the Live world, applied to actions instead of readings. A fake that records what it was asked to do is often called a **spy**.
