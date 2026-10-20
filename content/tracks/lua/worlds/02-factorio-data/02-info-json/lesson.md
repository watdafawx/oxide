**Why now:** a mod needs an ID card before its first prototype loads. `info.json` tells Factorio what the mod is called, which version it is and what it needs.

Four fields are mandatory: `name`, `version`, `title`, `author`. Add `factorio_version` (`"2.0"`, **two** numbers; a third is rejected by the mod portal) and `dependencies`. If you omit `dependencies` the default is `["base"]`.

```json
{
  "name": "belt-extras",
  "version": "1.2.0",
  "title": "Belt Extras",
  "author": "fox",
  "factorio_version": "2.0",
  "dependencies": ["base >= 2.0.0", "? some-optional-mod"]
}
```

- `name` is what Factorio uses internally. The mod portal allows letters, digits, `-` and `_`, 4 to 49 characters.
- `version` is `number.number.number`, each at most 65535.
- A dependency is `[prefix] name [operator version]`. No prefix means required, `?` optional, `!` incompatible.

Oxide cannot open `.json` files yet, so here you write the **text** of the file inside a Lua string in `src/info_json.lua`. The checker decodes it the way a JSON reader would. The boss ships a real `info.json`.

## Your turn

- Open `src/info_json.lua` and fill in the JSON text between the `[[` and `]]`.
