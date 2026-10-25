Oxide's engine is written in Rust, but the screen you are reading is a web page written in JavaScript. They are two programs, and a Rust `Profile` struct in memory means nothing to JavaScript. They need a way to write data down as text that both understand, and that text is JSON.

**Text both sides agree on.** **JSON** (JavaScript Object Notation) is a way of writing data as plain text with a few strict rules. Almost every language can read and write it, so it is how web pages, servers, save files and config files hand data to each other.

**Objects and arrays.** Curly braces hold an **object**: named values, `"key": value`, separated by commas, like the fields of a struct. Square brackets hold an **array**: values in order, like a `Vec`. They nest, so an object can hold an array of objects.

**Six kinds of value.** A JSON value is a string in double quotes (`"Loops"`), a number (`120`, `2.5`), `true` or `false`, `null` for "nothing here", an object, or an array. Keys are always strings in double quotes; single quotes and bare words are not JSON.

```json
{
  "xp": 340,
  "level": 3,
  "last_active": "2026-10-18",
  "completed": ["01-foundations/01-a-name-for-a-number"],
  "settings": { "sound": true, "focus_minutes": 25 },
  "best_time": null
}
```

**Mapping to Rust.** An object fits a struct, an array a `Vec`, a string a `String`, a number an integer or `f64`, `true`/`false` a `bool`. `null` fits an `Option` that is `None`, which is how JSON says "this field has no value".

**Two directions.** Turning a Rust value into JSON text is **serializing**; reading JSON text back into a Rust value is **deserializing**. Doing it by hand means hundreds of lines of string handling, so Rust programs use the **serde** crate, which writes that code for you from your struct. The next quest uses it.

## Your turn

Answer the three questions.
