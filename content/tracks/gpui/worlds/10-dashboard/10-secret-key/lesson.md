Kinetik's rule for the web dashboard: every request needs the secret key from the link shown in Settings, and New link replaces it. Without that, anyone on your network could open the page and watch what your PC is doing.

**A key in the link.** `new_key` makes 32 random hex characters from the operating system's random source. The link carries it after a `?`: `http://127.0.0.1:8787/?key=3f9c..`. The part after the `?` is the **query string**, `name=value` pairs the server can read.

**Read the query.** The handler's second input, `Query(param)`, already reads it into a `KeyParam`. Its `key` is an `Option<String>`: `None` when the link has no key at all. No key counts the same as a wrong key.

**Borrow the text inside.** `keys_match` takes two `&str`s. `.as_deref()` turns an `Option<String>` into an `Option<&str>`, a borrow of the text inside, without moving the `String` out. Then `.unwrap_or("")` makes a missing key the empty text, which never matches:

```rust
let name: Option<String> = Some("Kinetik".to_string());
let shown: &str = name.as_deref().unwrap_or("nobody"); // "Kinetik", or "nobody" for None
```

**Compare carefully.** A plain `==` may stop at the first different character. By timing many tries, an attacker learns how many characters were right and guesses the key piece by piece. `keys_match` always looks at every character, so the time says nothing.

**401 and nothing else.** A wrong key gets status 401, HTTP's "Unauthorized", with no data and no hint about the right key. Check at the very top of `stats` and `return StatusCode::UNAUTHORIZED.into_response()` early, before the snapshot is touched.

**Not in backups.** Kinetik leaves the key out of profiles and backups. Here it lives in `Settings`; if you share settings files, leaving it out is one line in `save`.

## Your turn

- At the top of `stats`, check the given key, or the empty text when there is none, against `state.key` with `keys_match`.
- When they do not match, return the 401 response. Save.
