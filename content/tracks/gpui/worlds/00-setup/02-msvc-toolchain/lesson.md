GPUI does not draw text and shapes itself. It hands them to Windows: DirectX draws, and DirectWrite lays out text. To link to those, Rust has to use Microsoft's build tools.

**Two Rust toolchains.** Rust on Windows comes in two flavours: `msvc`, which uses Microsoft's linker and the Windows SDK, and `gnu`, which does not. GPUI needs `msvc`.

**Check yours.** In a terminal, `rustup show` prints the active toolchain. You want a line ending in `x86_64-pc-windows-msvc`. Oxide's own Rust quests already built with it, so this is very likely fine.

**If it says gnu.** `rustup default stable-msvc` switches. The msvc toolchain needs Visual Studio Build Tools with "Desktop development with C++", which brings the Windows SDK.

**Nothing else to install.** GPUI comes from crates.io like any other library. The next quest adds it, and the first build starts when you save.
