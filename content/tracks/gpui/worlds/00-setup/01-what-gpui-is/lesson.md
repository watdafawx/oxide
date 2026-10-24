Kinetik Stats is a small Windows app: a strip of live numbers on the taskbar, a desktop widget, a game overlay. This track rebuilds it in Rust, one feature per quest, with a UI library called GPUI.

**What GPUI is.** GPUI is the library the Zed code editor draws its windows with. It is written in Rust, and you use it from Rust: no HTML, no designer tool.

**Describe, don't paint.** You never say "draw a grey rectangle at 10, 10". You describe what the window holds: a box with a grey background, holding the text `CPU 12%`. GPUI works out where everything goes, and the graphics card draws it.

**Every frame, fresh.** When something changes, such as a new CPU reading, your code describes the window again from scratch. GPUI lays that description out and repaints the window. You never patch the screen by hand.

**Why it suits Kinetik.** Kinetik redraws a few numbers every second and must stay tiny and fast. Drawing on the graphics card costs almost nothing, and Rust keeps memory use low.

**The first build.** The first quest that uses GPUI downloads about 200 small libraries and compiles them: about 2 minutes and 2 GB on disk, once. After that a save rebuilds in a few seconds, and every later quest reuses the same build.
