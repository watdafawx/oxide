The web dashboard is the same dashboard "in a browser on this PC or, if you allow it, other devices on your network". A browser cannot read Rust structs; it reads JSON. The bar's numbers have to become data first.

**Everything per stat.** `WebStat` has the kind, the short label, the text the bar would show, the raw value, and whether it is a warning. `#[derive(Serialize)]` lets `serde_json` write it.

**Every stat.** Unlike the bar, the web page shows every kind, read or not. Build a `Reading` per kind and take everything from it, so the web page and the bar never disagree about `13%` or `--`:

```rust
WebStat { kind, label: kind.label(), text: reading.text(), value: reading.value, warning: is_warning(&reading, &limits) }
```

**Text and value.** The page shows `text` as it is; `value` is there for anything that wants to chart or sort.

**Kinds as names.** serde writes each `StatKind` as its variant's name, `"CpuUsage"`, so the web page can tell stats apart without a lookup table.

## Your turn

- In `web_snapshot`, map every kind in `ALL` to a `WebStat` from its `Reading`.
- Give back `WebSnapshot { stats }`. Save.
