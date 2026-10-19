# Lua track: verified Factorio API notes

Checked 2026-10-19 against **lua-api.factorio.com/2.0.55** (pinned: the "latest" pages show 2.1.x), plus
`wube/factorio-data` (`core/lualib/*.lua`) where the docs are silent. Scope: world 2 (data stage) and
`content/tracks/lua/lib/mock_factorio.lua`. Control-stage names are NOT verified yet; verify them when
world 3 is written.

Base URL: `https://lua-api.factorio.com/2.0.55/`. "M" = mandatory, "o" = optional (default).

## Prototype pages

| Prototype / type | Page | Verified facts |
|---|---|---|
| `item` | `prototypes/ItemPrototype.html` | M: `type`, `name`, `stack_size`, `icon` or `icons`. o: `icon_size` (64), `place_result` (`""`), `subgroup`/`order` (no default listed: **not** mandatory), `fuel_value` ("0J"), `weight`. `stack_size` must be 1 with flag `not-stackable`. |
| `tool` (science packs) | `prototypes/ToolPrototype.html` | Inherits item. M extra: `durability` unless `infinite = true` (default false). |
| `recipe` | `prototypes/RecipePrototype.html` | M: `type`, `name` only. o: `ingredients`, `results` (both may be `{}`; neither mandatory), `energy_required` (0.5, must be > 0.001), `enabled` (true), `category` ("crafting"), `main_product`, `subgroup`, `order`. |
| ingredient | `types/ItemIngredientPrototype.html` | M: `type` (`"item"`), `name`, `amount` (uint16, non-zero). o: `ignored_by_stats` (0). Fluid variant fields not read; the mock accepts `type = "fluid"` with the same three. |
| result | `types/ItemProductPrototype.html` | M: `type` (`"item"`), `name`, and `amount` **or** `amount_min`+`amount_max`. o: `probability` (1, range 0..1). |
| `technology` | `prototypes/TechnologyPrototype.html` | M: `type`, `name`, `icon` or `icons`; `unit` **or** `research_trigger` (each mandatory if the other is missing). o: `icon_size` (64), `prerequisites` (array of tech names), `effects`, `enabled` (true), `max_level`. |
| `unit` | `types/TechnologyUnit.html` | M: `time` (double), `ingredients`. Exactly one of `count` / `count_formula`, "never both". |
| unit ingredient | `types/ResearchIngredient.html` | Tuple `{"automation-science-pack", 1}`; item must be a `tool`; amount non-zero. |
| unlock effect | `types/UnlockRecipeModifier.html` | `{type = "unlock-recipe", recipe = "<recipe name>"}`; `recipe` M. |
| `item-subgroup` | `prototypes/ItemSubGroup.html` | M: `type`, `name`, `group`. o: `order`. |
| `item-group` | `prototypes/ItemGroup.html` | M: `type`, `name`, `icon` or `icons`. o: `icon_size` (64), `order_in_recipe`, `order`. |
| `fluid` | `prototypes/FluidPrototype.html` | M: `type`, `name`, `default_temperature`, `base_color`, `flow_color`, `icon` or `icons`. |
| `container` | `prototypes/ContainerPrototype.html` | M: `type`, `name`, `inventory_size`. o: `inventory_type` ("with_bar"), `picture`. Everything inherited from entity pages is optional. |
| entity base | `prototypes/EntityPrototype.html` | M: `type`, `name`. `icon`/`icons` becomes mandatory when flags include `placeable-neutral`, `placeable-player` or `placeable-enemy`. `collision_box` default `{{0,0},{0,0}}`, `minable` default not minable. Flag `player-creation` only appears in a page example (not defined): quests use it only next to `placeable-neutral`, never test it. |
| `minable` | `types/MinableProperties.html` | M: `mining_time`. o: `result` (item name), `count` (1), `results`. |
| icon layer | `types/IconData.html` | M: `icon`. o: `icon_size` (64), `tint` (`{1,1,1,1}`), `shift`, `scale`. |
| colour | `types/Color.html` | `{r=,g=,b=,a=}` or `{r,g,b,a}` array of 3-4; floats 0..1; if any value > 1 all are read as 0..255. |
| file path | `types/FileName.html` | Forward slashes; must start with a root: `__core__`, `__base__` or `__<mod-name>__`. Extensions not specified on the page (examples are `.png`). |
| `ItemID` | `types/ItemID.html` | "The name of an ItemPrototype". (RecipeID/TechnologyID pages not read; same meaning is assumed and the mock only checks names exist.) |

## Data stage globals and functions

| Name | Source | Facts |
|---|---|---|
| `data.raw[type][name]` | `types/Data.html` | Dictionary of prototype type to (name to prototype). `data.raw.item["coal"].stack_size = 1000` is the documented edit idiom. |
| `data:extend(array)` | `types/Data.html` + `core/lualib/dataloader.lua` | Argument must be a non-empty array. Errors in the source: "Invalid array of prototypes", "Expected array of prototypes, but got a single prototype", "Invalid type in the following prototype definition", "Invalid name in the following prototype definition". Stores `data.raw[e.type][e.name] = e`: **a duplicate silently overwrites** (the duplicate checker is off by default). `data.extend{...}` works like `data:extend{...}`. The brief asked the mock to reject duplicates; the mock does not (fidelity), it records them in `mock.overwritten()` and the quests assert on that. |
| `mods` | `auxiliary/data-lifecycle.html` | Mod name to version for all enabled mods. |
| `settings.startup[name].value` | wiki Mod_settings, data-lifecycle page | Read-only snapshot at data stage. Writability from the data stage not documented: mock makes it read-only. |
| stage order | `auxiliary/data-lifecycle.html` | settings, settings-updates, settings-final-fixes, then data, data-updates, data-final-fixes. "Every mod goes through each round before the next round starts". Mod order: dependency depth, then natural sort of names. State is discarded between stages. |
| `table.deepcopy(t)` | `core/lualib/util.lua`; used in wiki modding tutorial | Global. Preserves the metatable (same reference), handles shared references and cycles. |
| `util.table.deepcopy` | `core/lualib/util.lua` | Alias: `util.table.deepcopy = table.deepcopy`. Needs `require("util")`. `util.copy` is the same alias. |
| `table_size(t)` | `auxiliary/libraries.html` | Global; counts like a `pairs` loop. |
| `log(x)`, `serpent` | `auxiliary/libraries.html` | `log` writes to the log file, example `log(serpent.block(data.raw["item"]["sulfur"]))` in the data stage. `serpent` is a global. |
| `require("util")` | `auxiliary/libraries.html` | Loads the core `lualib/util.lua`. |

## info.json

Source: `latest/auxiliary/mod-structure.html` (the 2.0.55 URL 404s).

| Fact |
|---|
| M: `name`, `version`, `title`, `author`. `factorio_version` is effectively mandatory (default `"0.12"`). o: `contact`, `homepage`, `description`, `dependencies` (default `["base"]`). |
| `version` = `number.number.number`, each 0..65535. `factorio_version` = `number.number` (`"2.0"`); a third number is rejected by the portal. |
| Portal name rule: only letters, digits, `-`, `_`; longer than 3 and shorter than 50 characters (game limit 100). |
| Dependency string: `"<prefix> name <op> version"`, ops `< <= = >= >`; prefixes `!` incompatible, `?` optional, `+` recommended, `(?)` hidden optional, `~` no load-order effect, none = required. |
| Zip name `{mod-name}_{version}`. |

## Dropped or changed because they could not be verified

| Idea | Decision |
|---|---|
| Quest 04 `icon_file_exists` | Dropped: the sandbox has no `io`, so a test cannot see files. Replaced by `icon_path_is_well_formed` (FileName rules, verified). |
| Boss `no_prerequisite_cycle` | The docs do not describe cycle handling. Replaced by `prerequisites_form_a_chain` (tests the learner's own tech order). |
| `mock.validate_references()` "place_result items match entities" | Kept as a mock lint: `place_result` must name an existing entity prototype; the docs say `place_result` is an entity name but not that load fails otherwise. Stated as a lint in error text. |
| `info.json` and `locale/*.cfg` as graded files | Not possible yet: `io`, `loadfile` and `dofile` are removed and `require` only loads `.lua`. Quest 02 grades the JSON text from `src/info_json.lua` with the mock's own small JSON decoder (`mock.decode_json`, not a Factorio API). The boss ships a real `mod/info.json` that is not graded. Locale is world 4. Engine follow-up: a read-only `oxide_read_file(relpath)` global scoped to the workspace would make real `.json`/`.cfg` gradable. |
| Dependency `~`, `(?)` handling | Verified, but the world-2 quest only needs `base`; richer dependency lint is world 4. |
| Mock validates at extend time | The game validates at the end of the stage (code may fill a field after `data:extend`). The mock does the same: `data:extend` only checks array/type/name; `mock.load_mod` runs `validate_all()` after the last round. |

## Not verified (kept, flagged)

- Icon paths such as `__base__/graphics/icons/iron-plate.png`, `wooden-chest.png`, `iron-gear-wheel.png` and `__base__/graphics/technology/steel-processing.png` follow the `FileName` rule and the vanilla layout, but file existence was not checked. The boss's optional in-game step is the real test.
- Vanilla fixture numbers (recipe costs, stack sizes, tech costs other than the `steel-processing` doc example) are close to vanilla, not exact. No quest depends on a vanilla number except `iron-gear-wheel` = 2 iron-plate and the `automation` / `steel-processing` names.
- `placeable-neutral` is verified (EntityPrototype page); `player-creation` only appears in an example there.
