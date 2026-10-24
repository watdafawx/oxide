A `serde_json::Value` can hold any JSON, but then every read is a guess: `v["size"]` might be missing, or a string. The downloader knows what an episode looks like: an id, a title and a size. It should say so once, as a struct, and let the JSON be checked against it.

**serde writes the reader.** **serde** is Rust's library for turning data into text and back. `#[derive(Deserialize)]` above a struct asks it to write the code that reads one from JSON, field by field, by name, the way `#[derive(Debug)]` wrote the printing code in world 02. That is why `use serde::Deserialize;` sits at the top.

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Show {
    pub name: String,
    pub episodes: u32,
}
// {"name": "The Daily", "episodes": 812}  ->  Show { name: "The Daily", episodes: 812 }
```

**Tell it the type.** `response.json().await` reads into whatever type is expected. When nothing around it says which, name it with the turbofish from world 03: `response.json::<Vec<Show>>().await` reads a JSON array of objects into a `Vec<Show>`.

**Checked, not guessed.** Each field must be there with the right kind of value: a missing `size`, or `"size": "big"`, makes the whole read an `Err` instead of a made-up `0`. Fields in the JSON that the struct doesn't have are skipped, so a server can add new ones without breaking old apps.

**For the curious.** A field that may be missing or `null` is written `Option<u64>`. `#[serde(rename = "fileSize")]` maps a JSON name onto a Rust field name, and world 10 adds more of these attributes and the other direction, `Serialize`.

## Your turn

- Add `Deserialize` to the derive list of `Episode`.
- In `fetch_feed`, make the request as in the last quest, then read the body as a `Vec<Episode>`.
