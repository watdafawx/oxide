The mod site's JSON was written long before your manager, and it spells things its own way: a mod's state is `"notinstalled"`, and each file says `"type": "resourcepack"`. Your Rust names are `ModStatus::NotInstalled` and a field called `kind`. serde can write one name in Rust and another in the JSON.

**Variants become strings.** A plain enum like `ModStatus` is written as the variant's name in a string: `ModStatus::NotInstalled` becomes `"NotInstalled"`. That is correct JSON, but the site compares against `"notinstalled"`, so every mod would look unknown.

**Rename them all.** `#[serde(rename_all = "lowercase")]` on the line above the enum changes how every variant is spelled: `NotInstalled` becomes `"notinstalled"`, `Enabled` becomes `"enabled"`. Other styles exist, such as `"snake_case"` for `"not_installed"`. Reading uses the same spelling, so `"notinstalled"` reads back into `ModStatus::NotInstalled`.

**A key Rust can't use.** The site wants a key called `type`, but `type` is a Rust keyword, so `pub type: String` does not compile. Call the field something else and rename only the key: `#[serde(rename = "type")]` above the field.

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Loader { Fabric, NeoForge }        // "fabric", "neoforge"

#[derive(Serialize, Deserialize)]
struct Pack {
    #[serde(rename = "mod-loader")]
    loader: Loader,                       // {"mod-loader": "fabric"}
}
```

**Both directions.** Every serde attribute applies to writing and reading alike, so one line keeps Rust and the other side in agreement. When the site changes its spelling, you change one attribute and no code. For the curious: `r#type` is a raw identifier that lets you name a field after a keyword, but most code prefers the rename.

## Your turn

- Make every `ModStatus` variant a lowercase word in the JSON.
- Make `ModFile`'s `kind` field appear as `type` in the JSON.
