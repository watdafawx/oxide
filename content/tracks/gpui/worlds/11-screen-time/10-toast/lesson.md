An alert becomes a Windows notification, the little card that slides in at the bottom right of the screen. Kinetik uses the `tauri-winrt-notification` crate, which talks to the same toast system as every other Windows app. Your part is the words on it.

**Short and plain.** A toast has a title and one line of text. The title names the problem; the text gives the value now, written by `Reading::text` like everywhere else, then "for 30 seconds or more.": `87°C for 30 seconds or more.`

**The titles.** Temperatures are "hot" ("GPU is hot"), usage is "busy" ("CPU is busy"), and the rest each get their own: "Ping is high", "Battery is low", "C: is nearly full". Any other kind falls back to "<label> warning".

**Kinds that share words.** `|` in a match pattern joins kinds that get the same words, as in the widget's groups. A last arm that is just a name, such as `other`, matches every kind left over and gives it that name, so the arm can still ask for its label:

```rust
let group = match kind {
    StatKind::NetDown | StatKind::NetUp => "Network".to_string(),
    other => format!("{} stats", other.label()),
};
```

**Showing it.** `show_toast` is written for you: `Toast::new(..).title(..).text1(..).show()`. A toast that fails to show is ignored, never a crash: a missing notification is better than a missing bar.

**Whose toast.** Windows shows toasts only for apps it knows. A portable exe is not registered, so Kinetik borrows PowerShell's app id, which every Windows PC has.

## Your turn

- Write `toast_text`: the title from a `match` on `alert.kind`, with the titles above.
- Give back the title and the value now as `Reading` text plus " for 30 seconds or more.".
- Save. Press Run and keep the GPU busy for half a minute to see one.
