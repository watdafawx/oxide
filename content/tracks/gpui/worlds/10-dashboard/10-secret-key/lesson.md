"Every request needs the secret key in the link shown in Settings, and New link replaces it." Without that, anyone on your network could watch your PC. The key is 32 random hex characters from the operating system (`new_key`), and the link carries it as `?key=...`.

**Read the query.** `Query<KeyParam>` gives the `key` parameter, if there is one. No key is the same as a wrong key.

**Compare carefully.** A plain `==` may stop at the first different character. Timing many tries, an attacker learns how many characters were right and guesses the key piece by piece. `keys_match` always looks at every character, so the time says nothing.

**401 and nothing else.** A wrong key gets `401 Unauthorized` with no data and no hint about the right key:

```rust
return StatusCode::UNAUTHORIZED.into_response();
```

**Not in backups.** Kinetik leaves the key out of profiles and backups. Here it lives in `Settings`; leaving it out of exports is a line in `save` if you share settings files.

## Your turn

- At the top of `stats`, check the given key (empty when missing) against `state.key` with `keys_match`.
- Return `StatusCode::UNAUTHORIZED.into_response()` when they differ.
- Save.
