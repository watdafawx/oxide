Before Factorio runs any of your Lua, it reads `info.json`, the mod's ID card. That file is not Lua: it is JSON, and a Lua habit inside it stops the mod from loading at all.

**A shared format.** **JSON** is a text format for data that almost every language can read. Factorio, the launcher and the **mod portal** (the website where mods are shared) all read `info.json`, so it uses JSON rather than Lua.

**Looks like a table.** A JSON object is a `{ }` of keys and values, a lot like a Lua table. But every key is in double quotes and a colon sits between key and value: `"name": "kit-mod"`, never `name = "kit-mod"`.

**No comma at the end.** Lua lets you leave a comma after the last entry. JSON does not, so the last line before `}` has no comma. Lists use square brackets, like `["base"]`.

```json
{
  "title": "Belt Extras",
  "tags": ["belts", "logistics"]
}
```

**Text inside Lua.** Oxide cannot open `.json` files yet, so the starter keeps the JSON as one long Lua string between `[[` and `]]`. Everything between the brackets, line breaks included, is the text, and the checker reads it the way Factorio would.

## Your turn

- In `info_json.lua`, fix the `name` line: quote the key and use a colon.
- Remove the comma after the last entry.
