The mod manager saves your modpack profile to disk, downloads mod lists from mod sites, and later shows everything on a web-style screen. Each of those is another program, or the same program next month, and a Rust struct in memory means nothing to them. They need a way to write data down as text that all sides understand, and that text is JSON.

**Text both sides agree on.** **JSON** (JavaScript Object Notation) is a way of writing data as plain text with a few strict rules. Almost every language can read and write it, so it is how launchers, mod sites, save files and config files hand data to each other.

**Objects and arrays.** Curly braces hold an **object**: named values, `"key": value`, separated by commas, like the fields of a struct. Square brackets hold an **array**: values in order, like a `Vec`. They nest, so an object can hold an array of objects.

**Six kinds of value.** A JSON value is a string in double quotes (`"sodium"`), a number (`120`, `2.5`), `true` or `false`, `null` for "nothing here", an object, or an array. Keys are always strings in double quotes; single quotes and bare words are not JSON.

```json
{
  "name": "All the Mods",
  "mod_count": 340,
  "last_played": "2026-10-18",
  "favourites": ["sodium", "iris"],
  "settings": { "auto_update": true, "keep_versions": 3 },
  "server": null
}
```

**Mapping to Rust.** An object fits a struct, an array a `Vec`, a string a `String`, a number an integer or `f64`, `true`/`false` a `bool`. `null` fits an `Option` that is `None`, which is how JSON says "this field has no value".

**Two directions.** Turning a Rust value into JSON text is **serializing**; reading JSON text back into a Rust value is **deserializing**. Doing it by hand means hundreds of lines of string handling, so Rust programs use the **serde** crate, which writes that code for you from your struct. The next quest uses it.

## Your turn

Answer the three questions.
