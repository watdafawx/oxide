An alert becomes a Windows notification, the little card that slides in at the bottom right. Kinetik uses the `tauri-winrt-notification` crate, which talks to the same toast system as every other Windows app.

**Short and plain.** A toast has a title and a line of text. The title names the problem; the text gives the value now, written by `Reading::text` like everywhere else.

**Kinds that share words.** `|` in a match pattern joins kinds with the same title, and a last arm with a binding covers the rest:

```rust
StatKind::CpuTemp | StatKind::GpuTemp => format!("{} is hot", alert.kind.label()),
```

**Showing it.** `show_toast` is written for you: `Toast::new(..).title(..).text1(..).show()`. A toast that fails to show is ignored, never a crash.

**Whose toast.** Windows shows toasts only for apps it knows. A portable exe is not registered, so Kinetik borrows PowerShell's app id, which every Windows PC has.

## Your turn

- Write `toast_text`: a title per kind, and the value with "for 30 seconds or more."
- Save. Press Run and make the GPU busy for half a minute to see one.
