Oxide's screen was written first, in JavaScript, and its JSON spells things its own way: a check status is `"notrun"`, and each quest card says `"type": "boss"`. The Rust names are `Status::NotRun` and a field called `kind`. serde can write one name in Rust and another in the JSON.

**Variants become strings.** A plain enum like `Status` is written as the variant's name in a string: `Status::NotRun` becomes `"NotRun"`. That is correct JSON, but the screen compares against `"notrun"`, so every check would look unknown.

**Rename them all.** `#[serde(rename_all = "lowercase")]` on the line above the enum changes how every variant is spelled: `NotRun` becomes `"notrun"`, `Passed` becomes `"passed"`. Other styles exist, such as `"snake_case"` for `"not_run"`. Reading uses the same spelling, so `"notrun"` reads back into `Status::NotRun`.

**A key Rust can't use.** The screen wants a key called `type`, but `type` is a Rust keyword, so `pub type: String` does not compile. Call the field something else and rename only the key: `#[serde(rename = "type")]` above the field.

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Theme { Dark, HighContrast }       // "dark", "highcontrast"

#[derive(Serialize, Deserialize)]
struct Prefs {
    #[serde(rename = "colour-theme")]
    theme: Theme,                          // {"colour-theme": "dark"}
}
```

**Both directions.** Every serde attribute applies to writing and reading alike, so one line keeps Rust and the other side in agreement. When the other side changes its spelling, you change one attribute and no code. For the curious: `r#type` is a raw identifier that lets you name a field after a keyword, but most code prefers the rename.

## Your turn

- Make every `Status` variant a lowercase word in the JSON.
- Make `QuestCard`'s `kind` field appear as `type` in the JSON.
