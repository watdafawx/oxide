Press Ctrl+Shift+Esc when a game hangs and you wait for Task Manager to appear. This track builds your own task manager in Rust: it aims to open at once, list every process, and stay lighter than Windows' own while it watches them.

**The job.** A Windows PC runs 200 to 400 processes. The window shows one row per process (name, CPU, memory), updates every second, and sorts when you click a column. That is the whole first screen.

**The first choice.** Rust has no built-in window toolkit, so you pick a library to draw with. The choice decides how you write every screen after this one, so this world tries each one on the same row before you commit.

**Route one: egui.** egui calls one function of yours to draw the whole window, every time something changes. A button is an `if`: it was clicked or it was not. Quick to learn, quick to build.

**Route two: GPUI.** GPUI is the library the Zed code editor is built with. Your screens are structs that GPUI keeps alive; when something changes, GPUI asks the struct to build its picture again. More to learn, built for big fast apps.

**Route three: the web.** Tauri shows a web page (HTML, CSS, JavaScript) in a window, and your Rust code does the work behind it. Oxide, the app you are reading this in, is built that way.

**How you will choose.** You build the same process row in egui and in GPUI, read how the web route works, then look at numbers measured on one Windows 11 PC: size, memory, CPU at rest, build time.
