# Track plan: Lua + Factorio

Runner `lua` (in-process Lua 5.2 via `mlua`, vendored). Five worlds, 54 quests, goal: the learner
ships a real Factorio 2.0 mod. This is a plan, not content. Format rules: `docs/CONTENT.md`.
Multi-track design: `docs/TRACKS.md`.

Folder layout: `content/tracks/lua/worlds/{01-lua-core,02-factorio-data,03-factorio-control,04-factorio-craft,05-capstone}/NN-slug/`.

Research snapshot (2026-10-19, from lua-api.factorio.com): Lua is **5.2.1** with changes; `global` was
renamed `storage` in 2.0; the API docs "latest" now show 2.1.x pages, so pin the targeted version
(2.0) in `track.toml` and re-check before content sessions (see Risks).

## 1. Runner and harness

### Quest shape

```
starter/
  tests.lua            visible spec; learner does not edit it
  src/*.lua            plain-Lua quests: the learner edits these
  mod/                 Factorio quests: info.json, data.lua, control.lua, locale/, migrations/ ...
solution/              same, all green (CI only)
```

Engine puts `<quest>/src`, `<quest>/mod` (as `__<modname>__`), and the engine's `oxide/` dir on
`package.path`, runs `tests.lua` inside a sandboxed Lua 5.2 state, and parses stdout with the same
parser as cargo (`test name ... ok`). Watcher extension: `lua` (+ `json`, `cfg` for Factorio quests).

Sandbox: instruction-count hook (5 s wall, kills `while true`), no `io`/`os`/`loadfile`/`dofile`
for Factorio quests (Factorio does not have them either); plain-Lua quests keep `os.time`/`os.clock`
only. Syntax errors become diagnostics with `file:line` from the Lua error text. Each test runs in
`pcall`; `mock.reset()` runs between tests in Factorio quests.

### `oxide/test.lua` API

```lua
local t = require("oxide.test")

t.test("name_of_objective", function() ... end)       -- one objective = one test
t.group("prefix_", function() t.test("x", ...) end)    -- optional name prefix, still flat output

t.eq(actual, expected, msg?)       -- primitive/identity compare; msg shown on failure
t.deep_eq(actual, expected, msg?)  -- structural table compare, ignores key order, prints a diff path
t.ne(a, b)  t.truthy(v)  t.falsy(v)  t.is_nil(v)
t.near(a, b, eps?)                 -- floats (eps default 1e-9)
t.is_type(v, "table")              -- type() check
t.contains(tbl_or_str, needle)     -- array membership / plain substring
t.raises(fn, pattern?)             -- fn must error; optional plain-substring match on message
t.skip(reason)                     -- reports `ignored`, not used by objectives

t.run()                            -- called last by tests.lua (or by the engine after it)
```

Output (cargo's shape, so one parser serves all runners):

```
running 3 tests
test sum_one_based ... ok
test append_and_pop ... FAILED
test length_after_removal ... ok

failures:

---- append_and_pop ----
src/arrays.lua:7: expected 3, got nil
  hint: Lua arrays start at 1

test result: FAILED. 2 passed; 1 failed; 0 ignored
```

Failure text uses `file:line` of the **learner's** frame (walk the traceback, skip `oxide/*`). A
test that errors before `t.run()` (syntax error, missing `require`) produces no `test ...` lines:
the engine reports "did not run" plus the error, like a cargo compile failure.

Why not busted: busted needs LuaRocks and a system Lua; the sandbox rules (5.2, no `os`) and the
cargo-shaped output are easier to own. The API above is deliberately a tiny subset of busted/luassert
names (`eq`/`deep_eq`/`truthy`/`raises`) so the habits transfer.

### `mock_factorio.lua`

`local mock = require("mock_factorio")` in Factorio quests. It installs the Factorio globals
(`data`, `settings`, `defines`, `script`, `game`, `storage`, `remote`, `commands`, `prototypes`,
`helpers`, `log`, `serpent`, `table.deepcopy`, `table_size`, `util`) and returns a control object.
Everything is a **plain Lua table** the learner can print, so failures are readable.

| Group | Must implement |
|---|---|
| Harness (`mock.*`) | `reset()`, `load_mod(dir)` (runs settings → data → data-updates → data-final-fixes in Factorio order, then control.lua), `new_game()` (runs `on_init`), `save_load()` (serializes `storage`, errors on functions, drops unregistered metatables, reruns control.lua then `on_load`), `tick(n)`, `fire(event_name, data)`, `add_player(name?)`, `place(name, pos, opts?)` (builds an entity + fires `on_built_entity`), `finish_research(name)`, `click(gui_element)`, `printed()` (captured `game.print`/`player.print` lines), `stats` (API call counters), `two_peers(fn)` (run the same script on two games, compare a hash of `storage`) |
| Data stage | `data.raw[type][name]`; `data:extend{...}` with **validation** of mandatory fields for the ~15 prototype types the course uses (`item`, `recipe`, `technology`, `container`, `assembling-machine`, `item-group`, `item-subgroup`, `mod-setting` types, `custom-input`, `sprite`, `tool`, `capsule`, `lab`, `furnace`, `generator`?). Re-extend overwrites like the game. A **vanilla fixture** pre-fills `data.raw` with a small real subset (iron-plate, copper-plate, iron-gear-wheel, stone-furnace, wooden-chest, assembling-machine-1, automation / logistics / steel-processing technologies, automation-science-pack, lab) so `table.deepcopy` / `data.raw` edits are meaningful. `mods` table, read-only `settings.startup`, `mock.validate_references()` (every recipe ingredient/result, tech prerequisite and unlock effect resolves; no tech cycle; `place_result` items match entities), graphics path check (`__modname__/...` prefix, file exists in `mod/graphics/` when the quest ships one) |
| Settings stage | `data:extend` of `bool-setting` / `int-setting` / `double-setting` / `string-setting` with `setting_type = startup / runtime-global / runtime-per-user`, `default_value`, `allowed_values`, min/max checks. `mock.set_setting(scope, name, value)` |
| Control: lifecycle | `script.on_init`, `on_load` (storage becomes **read-only**: writes raise, like the game), `on_configuration_changed` (`mock.change_config{mod_changes=...}`), `on_event(events, handler, filters?)`, `on_nth_tick`, `register_metatable`, `register_on_object_destroyed`, `active_mods`, `mod_name`. `control.lua` body runs with `game` = nil (matches the game: no `game` while control.lua loads) |
| Control: events | `defines.events` (subset: `on_tick`, `on_player_created`, `on_built_entity`, `on_robot_built_entity`, `on_player_mined_entity`, `on_entity_died`, `on_object_destroyed`, `on_gui_click`, `on_gui_closed`, `on_runtime_mod_setting_changed`, `on_research_finished`, `on_player_joined_game`, `on_string_translated` is **out**). Event filters on build/mine events honoured (name/type) |
| Control: game | `game.tick`, `game.print`, `game.players`, `get_player(i/name)`, `game.surfaces`, `game.forces`, `game.get_surface`, `create_random_generator`, `get_entity_by_unit_number`, `game.speed` (settable, ignored) |
| Surface/entity stubs | `surface.create_entity{name, position, force}` returns a **stub entity table**: `name`, `type` (looked up in `data.raw` if loaded), `position`, `force`, `surface`, `unit_number` (counter), `valid`, `destroy()`, `health`; `find_entities_filter{area/position+radius, name, type, force}`, `count_entities_filter`; resource entities for ore quests via `mock.place("iron-ore", pos, {amount=500})`. No collision, no pathing, no power, no fluids, no crafting |
| Players/GUI | `LuaPlayer`: `index`, `name`, `connected`, `print`, `gui.{top,left,screen,relative}`, `get_inventory`/`insert`/`remove_item`/`get_item_count` over a flat `{name→count}` bag. `LuaGuiElement`: `add{type,name,caption,tags,direction,...}` for `frame`, `flow`, `label`, `button`, `sprite-button`, `textfield`, `checkbox`, `table`; `children`, `children_names`, `[name]` lookup, `destroy()`, `valid`, `caption`, `visible`, `tags`. `mock.click(el)` raises `on_gui_click`. No layout/pixel positions |
| Storage/remote/commands | `storage` table (rules above); `remote.add_interface/call/interfaces` (`call` of a missing interface raises with the game's message); `commands.add_command(name, help, fn)` + `mock.run_command(name, "args", player)`; `helpers.table_to_json` / `json_to_table`; `helpers.compare_versions`; `log` captured; `serpent.block/line` (tiny real impl or vendored) |
| Packaging helpers (craft world) | `mock.read_json(path)`, `mock.lint_info(dir)` (name charset/length, version `a.b.c` ≤ 65535, `factorio_version = "2.0"`, dependencies syntax incl. `?`, `!`, `~`), `mock.parse_changelog(path)` (strict header/dash format), `mock.load_locale(dir, lang)` (cfg sections, `__1__` placeholders, key per prototype), `mock.zip_name_ok(dir)` |

Faked on purpose (the learner is told in the Setup read): `create_entity` returns a stub; `pairs` order
is **not** insertion order (Factorio's is; PUC Lua's is hash order, so tests never depend on it and
a lesson says "never depend on it in the game either, use arrays when order matters"); `math.random`
is plain Lua's (Factorio's is deterministic — the multiplayer quest uses `game.create_random_generator`);
no real time; `on_tick` is called only by `mock.tick`; research has no progress, only `finish_research`.

### Cannot be auto-graded

- Graphics: icon exists and path shape is checked, **looks right** is not.
- Gameplay feel: balance, recipe cost sanity, tech tree position, GUI layout and style, sounds.
- Interaction with other mods and real engine behaviour: collision boxes, power, inserters, robots.
- Real desyncs. The two-peer mock catches the classic mistakes (`on_load` writes, local-only state), not all.
- Real performance: the mock counts API calls and loop iterations; wall time in Factorio is not measured.

### "Load it in Factorio, then quiz" pattern

At each world boss and in the capstone: the lesson ends with 3 steps (copy the `mod/` folder to the
mods dir via a button, start Factorio, do one specific thing). The follow-up `read` quest asks a
question only someone who saw it can answer ("What does the tooltip of the Survey Kit say?",
"How many lines does the console print after you place the beacon?"). XP is 15 (read), the badge
"Seen in game" is the reward. If Factorio is not installed the quest is skippable and shows no lock.

Optional dry run (Factorio found via `FACTORIO_EXE`, else default Steam paths, else off — it is not on
this machine's default paths today): the engine runs
`factorio --mod-directory <tmp> --create <tmp>/t.zip`, then `--benchmark t.zip --benchmark-ticks 120`
(headless load + 120 ticks), reads the log for `Error`/`Failed to load mod`, and shows pass/fail as a
bonus objective named `loads_in_factorio` that never blocks XP. `factorio --dump-data` can export the
real `data.raw` for a diff against the mock's validation. Flags: verify with `factorio --help` when
implementing. The user's own harness in `X:\Dev\factorio` (`mods/*/test/run.py`, MCP "factorio") already
does this per mod; reuse its detection logic (`factorio_paths.py`), do not depend on it.

### Existing tools surveyed (what the mock borrows)

- **Testorio / factorio-test** (npm): runs tests **inside** a real game, busted-like API. Most faithful,
  needs Factorio + Node: not usable as the default grader, good model for the optional dry run.
- **FUnit, factorio-check**: in-game test mods driven from outside. Same trade-off.
- **Faketorio** (LuaRocks): helper to build/run mod tests outside the game; limited mocking.
- No maintained "mock the whole API under busted" project found. Most mod authors stub `game`, `script`,
  `defines` by hand. So `mock_factorio.lua` is ours; keep it small and honest rather than complete.
- Scope rule from the user's own mods (AGENTS.md there): "test what the change can break". The mock
  tests logic (events, storage, data validity), the real game tests the rest.

## 2. Factorio 2.0 facts the lessons teach (checked against lua-api.factorio.com)

| Topic | Fact |
|---|---|
| Lua | 5.2.1 plus changes: no `io`, `os`, `coroutine`, `loadfile`, `dofile`; `debug` only `getinfo`/`traceback`; `pairs` iterates in insertion order (first 1024 integer keys 1..1024); `math.random` deterministic, `math.randomseed` no-op (use `LuaRandomGenerator`); `string.pack/unpack` backported from 5.4; `require("util")` = core lualib; `require("__mod__.file")` |
| Stages | settings (`settings.lua`, `-updates`, `-final-fixes`) → data (`data.lua`, `data-updates.lua`, `data-final-fixes.lua`; `data`, `mods`, read-only `settings`; no game API) → control (`control.lua`, no `game` in the body) |
| Control order | `control.lua` → `on_init` (new save or newly added mod) → migrations (JSON first, then Lua, once per save) → `on_load` (no `game`, `storage` read-only; only re-link metatables / local refs / conditional subscriptions) → `on_configuration_changed` (all or none) |
| `storage` | Renamed from `global` in 2.0. Serialized with the save; functions forbidden; metatables lost unless `script.register_metatable`; not restored during `control.lua`, writing in `on_load` errors |
| Other 2.0 moves to mention | `game.*_prototypes` → global `prototypes`; `game.table_to_json` etc. → `helpers`; recipe `ingredients`/`results` entries are `{type="item", name=..., amount=...}` (the old `{name, n}` array form is gone); `script.register_on_object_destroyed` (was entity). Lessons use the 2.0 form only, with one callout "tutorials before October 2024 use `global` / old recipe syntax" |
| `info.json` | Required: `name`, `version`, `title`, `author`. Optional: `description`, `homepage`, `contact`, `dependencies`, `factorio_version` (`"2.0"`, two numbers only), Space Age flags. Dependency prefixes `!` `?` `(?)` `+` `~`. Portal name rule: 4-49 chars, `[A-Za-z0-9_-]`. Zip = `name_version.zip` |
| Migrations | `migrations/*.json` (rename prototypes) and `migrations/*.lua` (fix saved state); sorted by mod then file name; each runs once |
| Portal | Upload release zip on mods.factorio.com (web or the API with an API key); changelog.txt is parsed strictly; thumbnail 144x144 |

## 3. Worlds

XP: exercise 30-80, read 15, boss 300-500. QW = quick win (easy, low XP/minutes, keeps the streak).
Boss is last in its world. Objective names are the `tests.lua` test names; reads list "quiz (n)".

| # | World | Quests | Time | XP | Boss |
|---|---|---|---|---|---|
| 1 | Lua core | 12 | 152 min | 905 | Expression evaluator |
| 2 | Factorio data stage | 11 | 127 min | 845 | A new item, recipe and tech chain |
| 3 | Factorio control stage | 12 | 173 min | 1025 | A reactive mod with GUI |
| 4 | Factorio craft | 10 | 115 min | 735 | Harden a sloppy mod, ready for the portal |
| 5 | Capstone | 9 | 181 min | 935 | "Ore Ledger", a full mod |
| | **Total** | 54 | 748 min | 4445 | |

### World 1 — Lua core (`01-lua-core`) — plain Lua, no mock, `requires` nothing

Gap-fill starters: `src/*.lua` fills `error("TODO")`. Factorio is mentioned only in one-line "you
will use this when..." hooks.

| slug | idea | objectives (tests) | min | XP | kind |
|---|---|---|---|---|---|
| 01-setup | what the runner does, edit-save-test loop, `print`, Lua 5.2 vs the tutorials you will find | quiz (2) | 3 | 15 | read |
| 02-values-and-numbers | nil/false only falsy, `..`, `tostring`, 5.2 has **no integers**: `7/2`, `%`, `math.floor`, `string.format("%d")` | `only_nil_and_false_are_falsy`, `halves_and_floors_correctly`, `formats_whole_numbers` | 5 | 30 | QW |
| 03-arrays | 1-based tables, `table.insert/remove/concat/sort`, `#`, ipairs | `sum_one_based`, `append_and_pop`, `sort_descending`, `join_with_commas` | 8 | 45 | exercise |
| 04-holes-and-nil | nil in tables, `#` on holes, `select('#', ...)`, `table.pack`/`unpack`, why `{n=...}` | `length_survives_removal`, `counts_trailing_nils_with_select`, `compacts_sparse_list`, `pack_keeps_nils` | 8 | 45 | exercise |
| 05-functions-and-varargs | multiple returns, defaults via `or`, `...`, `select`, argument-count traps | `returns_min_and_max`, `default_arg_with_or`, `sum_varargs`, `keeps_false_argument` | 10 | 50 | exercise |
| 06-closures | upvalues, counters, factories, loop-variable capture, `local` vs global leak | `counter_counts_independently`, `loop_closures_capture_each_value`, `memoize_calls_once` | 8 | 50 | exercise |
| 07-dicts-and-iteration | `pairs` vs `ipairs`, `next`, deleting while iterating, shallow vs deep copy, key order is not yours to rely on | `counts_words`, `removes_during_pairs_safely`, `deep_copy_is_independent` | 8 | 35 | exercise |
| 08-string-patterns | `find/match/gmatch/gsub`, `%d+`, captures, anchors, `-` vs `*`, escaping, `string.format` | `parses_key_value_line`, `splits_on_commas`, `replaces_with_function`, `escapes_magic_chars` | 12 | 60 | exercise |
| 09-metatables-and-oop | `__index`, `__newindex`, `__call`, `__tostring`, `__eq`/`__lt`, `setmetatable` classes, `:` vs `.` | `instances_share_methods`, `read_only_table_rejects_writes`, `vector_adds_with_plus`, `tostring_formats_vector` | 12 | 70 | exercise |
| 10-modules-and-errors | `require`, module-returns-table, no accidental globals (strict `_ENV`), `error` with tables, `pcall`/`xpcall`, `assert` | `module_exposes_only_public_api`, `catches_error_object`, `rethrows_with_context`, `finds_accidental_global` | 6 | 35 | QW |
| 11-coroutines | `create/resume/yield/wrap/status`, generators, producer-consumer. Callout: **Factorio has no coroutines**; here as a language idea and for the boss | `generator_yields_in_order`, `passes_values_both_ways`, `round_robin_runs_tasks` | 12 | 70 | exercise |
| 12-boss-expression-evaluator | tokenizer (coroutine/generator) → recursive-descent parser → evaluator with variables and error messages | `tokenizes_numbers_and_operators`, `respects_precedence`, `handles_parentheses_and_unary_minus`, `reads_variables_from_environment`, `reports_syntax_error_with_position`, `rejects_division_by_zero` | 60 | 400 | boss |

### World 2 — Factorio data stage (`02-factorio-data`) — `requires` 01-lua-core/23-catch-the-error

Rewritten into 18 quests; see `docs/REWRITE-lua-02.md` for the current list. The table below is the original plan.

Mock: data stage + settings stage. Each starter has a `mod/` folder; `tests.lua` calls
`mock.load_mod("mod")` and asserts on `data.raw`.

| slug | idea | objectives (tests) | min | XP | kind |
|---|---|---|---|---|---|
| 01-setup | folder layout, the three stages in order, why `data.lua` has no `game`; optionally set the Factorio path | quiz (2) | 4 | 15 | read |
| 02-info-json | write a valid `info.json`: name, version, `factorio_version = "2.0"`, dependencies | `has_required_fields`, `factorio_version_is_2_0`, `depends_on_base` | 6 | 35 | QW |
| 03-first-item | `data:extend` an `item`: `name`, `icon`, `stack_size`, `subgroup`, `order` | `item_is_registered`, `item_has_mandatory_fields`, `stack_size_is_set` | 8 | 50 | exercise |
| 04-icons-and-paths | `__modname__/graphics/x.png`, `icon_size`, `icons` layers and tint | `icon_path_uses_mod_prefix`, `icon_file_exists`, `tints_with_icons_layer` | 4 | 30 | QW |
| 05-recipes | 2.0 `ingredients`/`results` tables, `energy_required`, `enabled`, `category`, `main_product` | `recipe_ingredients_use_2_0_format`, `recipe_result_is_the_item`, `crafting_time_is_two_seconds` | 10 | 55 | exercise |
| 06-copy-and-tweak | `table.deepcopy(data.raw...)`, rename, change fields, never mutate the original | `copy_is_independent`, `original_is_untouched`, `new_name_is_unique` | 10 | 60 | exercise |
| 07-technologies | `technology`: `effects` (`unlock-recipe`), `prerequisites`, `unit`, recipe `enabled = false` | `tech_unlocks_the_recipe`, `prerequisites_exist`, `unit_uses_valid_science_packs`, `recipe_starts_disabled` | 12 | 70 | exercise |
| 08-stage-order | `data` vs `data-updates` vs `data-final-fixes`: change vanilla late, other mods' changes visible | `final_fixes_sees_other_prototypes`, `updates_run_after_data`, `change_is_applied_once` | 8 | 45 | exercise |
| 09-first-entity | a chest-like `container` with `minable`, `place_result`, `collision_box`, `inventory_size` | `entity_is_defined`, `item_places_the_entity`, `minable_returns_the_item`, `box_is_symmetric` | 12 | 70 | exercise |
| 10-reading-the-prototype-docs | how to read lua-api prototype pages: mandatory fields, inheritance (`type` parent chain), types of values | quiz (3) | 3 | 15 | read |
| 11-boss-new-item-chain | one mod: raw item → intermediate → final item + entity, 2 recipes, a 2-tech chain, locale-ready names. Then "load it in Factorio" lesson | `all_prototypes_register`, `references_resolve`, `tech_chain_unlocks_both_recipes`, `no_prerequisite_cycle`, `entity_item_matches_place_result`, `icons_are_valid` | 50 | 400 | boss |

The boss lesson ends with an optional manual "load it in Factorio" step; the quiz for it lives in the capstone.

### World 3 — Factorio control stage (`03-factorio-control`) — `requires` 02-factorio-data/14-the-research-bill

| slug | idea | objectives (tests) | min | XP | kind |
|---|---|---|---|---|---|
| 01-setup | what `control.lua` can and cannot do at load, `game` appears only inside handlers, the event model | quiz (2) | 4 | 15 | read |
| 02-script-on-event | `script.on_event(defines.events.x, fn)`, event table fields, `game.print` | `handler_is_registered`, `handler_prints_greeting`, `uses_event_player_index` | 8 | 50 | exercise |
| 03-commands-and-players | `commands.add_command`, args, `game.get_player`, `player.print` | `command_is_registered`, `command_prints_argument`, `unknown_player_is_nil_checked` | 5 | 30 | QW |
| 04-storage | per-mod `storage`, what survives a save, no functions, tables of players | `counter_persists_through_save_load`, `defaults_created_once`, `function_in_storage_is_rejected` | 10 | 55 | exercise |
| 05-init-load-config | `on_init` / `on_load` / `on_configuration_changed`, what each may touch | `on_init_sets_defaults`, `on_load_does_not_write_storage`, `config_changed_adds_missing_field` | 10 | 60 | exercise |
| 06-entities-and-surfaces | `surface.create_entity`, `find_entities_filter`, `entity.valid`, `destroy`, unit numbers | `creates_entity_at_position`, `finds_by_name_and_radius`, `skips_invalid_entities`, `destroyed_entity_is_gone` | 12 | 65 | exercise |
| 07-event-filters-and-tracking | build/mine/die events with filters; track entities in `storage` by `unit_number`; cleanup with `register_on_object_destroyed` | `only_listed_entities_fire`, `tracks_built_entity`, `forgets_destroyed_entity` | 10 | 55 | exercise |
| 08-tick-budgeting | `on_tick` vs `on_nth_tick`, process N items per tick round-robin, never scan everything per tick | `never_exceeds_budget_per_tick`, `covers_all_items_over_time`, `uses_nth_tick_not_every_tick` | 12 | 70 | exercise |
| 09-gui | `player.gui.screen.add` frame + button + label, `on_gui_click`, `tags`, close and destroy | `frame_has_expected_children`, `click_updates_label`, `close_destroys_frame`, `no_duplicate_frame_on_second_open` | 14 | 75 | exercise |
| 10-remote-interfaces | `remote.add_interface` / `call`, exposing read-only data, versioned interface | `interface_is_registered`, `call_returns_value`, `missing_interface_errors_cleanly` | 8 | 40 | QW |
| 11-migrations | `migrations/*.lua` vs `.json`, upgrading old saved shape, idempotent | `migration_renames_field`, `migration_is_idempotent`, `json_migration_maps_old_name` | 10 | 60 | exercise |
| 12-boss-reactive-mod | a mod that reacts to events: counts built/mined entities, GUI panel, command, remote interface, budgeted tick, survives `save_load` | `counts_built_entities`, `survives_save_load`, `gui_shows_counts`, `button_resets_counter`, `command_prints_totals`, `remote_returns_totals`, `tick_work_is_budgeted`, `on_load_is_pure` | 70 | 450 | boss |

### World 4 — Factorio craft (`04-factorio-craft`) — `requires` 03-factorio-control/09-gui

| slug | idea | objectives (tests) | min | XP | kind |
|---|---|---|---|---|---|
| 01-craft-checklist | what separates a hack from a mod people install: settings, locale, compat, performance, multiplayer, portal | quiz (2) | 3 | 15 | read |
| 02-mod-settings | `settings.lua` startup / runtime-global / runtime-per-user; read in data vs control; react to `on_runtime_mod_setting_changed` | `setting_prototypes_are_valid`, `data_stage_reads_startup_setting`, `runtime_change_is_handled`, `per_user_setting_is_per_player` | 10 | 60 | exercise |
| 03-locale | `locale/en/*.cfg`, `[item-name]`, `__1__` params, one key per prototype, `localised_string` | `every_prototype_has_a_name`, `placeholders_match_arguments`, `no_unused_keys` | 8 | 45 | exercise |
| 04-changelog | `changelog.txt` strict format, version header, categories, dates | `header_format_is_valid`, `latest_matches_info_version`, `categories_are_known` | 5 | 30 | QW |
| 05-performance | cache lookups, avoid per-tick scans, batch work, look at the mock's API-call counter | `no_per_tick_surface_scan`, `api_calls_stay_under_budget`, `cache_is_invalidated_on_change` | 12 | 65 | exercise |
| 06-multiplayer-safety | determinism: `LuaRandomGenerator`, no local-only state, `on_load` rules, desync classics, run on two peers | `uses_game_random_generator`, `peers_end_with_equal_storage`, `no_state_outside_storage`, `no_os_or_io` | 12 | 65 | exercise |
| 07-compat-and-dependencies | `dependencies` with `?` and `!`, `script.active_mods`, optional integration, `data-final-fixes` | `works_without_optional_mod`, `uses_optional_mod_when_present`, `incompatible_mod_is_declared` | 10 | 55 | exercise |
| 08-portal-lint | name/version/zip rules, thumbnail, description, dependencies sanity | `name_is_portal_valid`, `zip_name_matches_version`, `thumbnail_and_changelog_present` | 6 | 35 | QW |
| 09-publishing | portal upload (web and API key), version bumps, never reuse a version, deprecating, reading user reports | quiz (3) | 4 | 15 | read |
| 10-boss-harden-a-mod | given a working but sloppy mod, fix settings, locale, tick budget, `on_load` writes, optional dep, portal lint | `settings_replace_magic_numbers`, `all_names_localised`, `tick_handler_is_budgeted`, `on_load_is_pure`, `optional_dep_is_guarded`, `portal_lint_is_clean` | 45 | 350 | boss |

### World 5 — Capstone (`05-capstone`) — `requires` 04-factorio-craft/08-portal-lint (and 03 boss)

The mod: **Ore Ledger**. A research-gated *Survey Beacon* (new item, recipe, 2-tech chain) that, once
built, scans the ore around it (one beacon per few ticks), keeps totals in `storage`, shows them in a
GUI panel, exposes `remote.call("ore-ledger", "totals")`, has a startup setting for radius and a
runtime setting for scan interval. The learner builds it across the world; each quest owns files.

| slug | idea | objectives (tests) | min | XP | kind |
|---|---|---|---|---|---|
| 01-design | read the one-page spec, file map, who owns what; sketch the data flow | quiz (2) | 4 | 15 | read |
| 02-scaffold | `info.json`, folders, empty `data.lua` / `control.lua` that load | `mod_loads_clean`, `info_lint_passes` | 8 | 40 | exercise |
| 03-data-chain | beacon item + entity + recipe + 2 techs + settings prototypes | `beacon_prototypes_register`, `techs_unlock_in_order`, `startup_setting_exists`, `references_resolve` | 15 | 80 | exercise |
| 04-scanning | on build: find ore in radius, count by resource, store per beacon, forget on destroy | `scan_counts_ore_in_radius`, `totals_stored_per_beacon`, `destroyed_beacon_is_forgotten` | 15 | 80 | exercise |
| 05-tick-and-settings | rescan on a budget, interval from runtime setting, reacts to setting change | `scans_one_beacon_per_interval`, `interval_follows_setting`, `survives_save_load` | 12 | 65 | exercise |
| 06-gui-and-remote | totals panel with refresh and close; remote `totals` | `panel_lists_every_resource`, `refresh_button_rescans`, `remote_totals_matches_storage` | 15 | 80 | exercise |
| 07-polish | locale, changelog, portal lint, no `on_load` writes, two-peer equality | `all_names_localised`, `portal_lint_is_clean`, `peers_end_with_equal_storage` | 12 | 60 | exercise |
| 08-play-it | button copies the mod into the Factorio mods folder, launch, place a beacon, read the panel; quiz on what you saw. Optional `loads_in_factorio` dry run shows here | quiz (3); bonus `loads_in_factorio` | 10 | 15 | read |
| 09-boss-ship-ore-ledger | the whole mod against the full suite: data, control, GUI, settings, remote, migration stub for v0.2, lint | `data_stage_valid`, `scans_and_stores`, `budgeted_scanning`, `gui_matches_storage`, `remote_interface_works`, `settings_respected`, `migration_adds_new_field`, `survives_save_load`, `peers_equal`, `portal_lint_clean` | 90 | 500 | boss |

## 4. Lua gotchas → quest that teaches each

| Gotcha | Why it bites in Factorio | Quest |
|---|---|---|
| Arrays are 1-based | Everything (`player_index`, inventories) starts at 1 | 1/03-arrays |
| Only `nil` and `false` are falsy (`0` and `""` are truthy) | `if count then` is always true | 1/02-values-and-numbers |
| No integer type in 5.2; `/` is always float, no `//` | `string.format("%d", 2.5)` errors; ticks as floats | 1/02-values-and-numbers |
| `nil` in tables, `#` on tables with holes | Removing `storage.x[i] = nil` breaks `#storage.x` | 1/04-holes-and-nil |
| Varargs: `select('#', ...)`, `...` in the middle truncates | Event wrappers, `table.pack` | 1/05-functions-and-varargs |
| Multiple returns truncated in the middle of a list | `{f()}` vs `{f(), 1}` | 1/05-functions-and-varargs |
| Closures capture variables, not values; fresh `local` per loop iteration | Handler closures over loop vars | 1/06-closures |
| Forgotten `local` makes a global | Factorio warns/crashes in some contexts; leaks between mods' envs | 1/10-modules-and-errors |
| `pairs` order | Unspecified in plain Lua; Factorio uses insertion order — don't rely on it | 1/07-dicts-and-iteration |
| Modifying a table while iterating | Deleting entities from a tracked list | 1/07-dicts-and-iteration |
| `:` vs `.` call syntax | `obj.method()` loses `self` | 1/09-metatables-and-oop |
| Metatables are not saved | Class instances in `storage` lose methods after load | 1/09, 3/04-storage, 3/05-init-load-config |
| No `os`, `io`, `coroutine` in Factorio | Lessons run in a sandbox that lacks them where it matters | 1/11-coroutines, 4/06-multiplayer-safety |
| String patterns are not regex | `-` lazy, `%` escape, no alternation | 1/08-string-patterns |
| `error` with a table, `pcall` returns `false, err` | Safe calls into other mods' remotes | 1/10-modules-and-errors |
| `game` is nil while `control.lua` loads | Using `game.*` at file top level crashes | 3/01-setup, 3/05-init-load-config |
| Writing `storage` in `on_load` | Desync; the mock raises like the game | 3/05, 4/06 |
| Invalid LuaObjects | `entity.valid` must be checked after any tick | 3/06-entities-and-surfaces |
| Old `global` / old recipe syntax from tutorials | 2.0 uses `storage` and `{type,name,amount}` | 2/05-recipes, 3/04-storage |

## 5. Risks and open questions

| Risk | Plan |
|---|---|
| Mock fidelity: the mock can pass code the game rejects, or fail code the game accepts | Keep it small, document what is faked in the Setup read, validate only the mandatory fields; `loads_in_factorio` dry run when Factorio is present; CI compares the mock's data validation against a `--dump-data` of the solutions when Factorio is available |
| API churn: docs "latest" already shows 2.1.x pages | Pin `factorio_version = "2.0"`; a content-check pass lists every API name the mock implements, with the doc version; re-check before each world's content session; keep `docs/tracks/lua-api-notes.md` (to create) with verified names |
| `pairs` ordering differs from Factorio | Tests never depend on it; lesson says so; optional: replace `pairs` in the sandbox with an insertion-ordered one for Factorio quests (cheap to do with a tracking `__newindex` proxy but slows and breaks `rawset` — decide at implementation) |
| `mlua` Lua 5.2 feature set | Confirm vendored 5.2 has `goto`, `bit32`; `string.pack` is **not** in 5.2 (Factorio backports it): ignore in lessons or add a pure-Lua shim |
| Names I could not verify offline | `helpers.*` contents, exact event names in the subset, `script.register_on_object_destroyed` signature, `LuaGuiElement` property set. Verify each against lua-api when writing the mock; delete from the quest idea rather than guess |
| GUI quests are shallow without real rendering | They grade structure (names, children, click effects) only; the boss lesson links the in-game check |
| Capstone dry run needs the game; Factorio not in default paths here | Optional bonus objective; never blocks XP; detection order `FACTORIO_EXE` → Steam default → settings |
| Learners may copy solutions from the Factorio wiki / other mods | Fine; objectives are about behaviour; bosses are multi-test and `survives_save_load`/`peers_equal` need real understanding |
| Lua core can feel unrelated to the goal | Each lesson opens with a one-line Factorio hook; the 12-quest world stays short; quick wins every third quest |
| Where does "play it" go for worlds 2-4 | Only the capstone has a dedicated `read` quest; boss lessons end with an optional manual step |
| Is `requires` by world boss too strict | World 2 requires only Lua core `10-modules-and-errors`, so a learner can start Factorio before finishing coroutines and the evaluator. Revisit after playtest |
| Open question: second language file for locale | English only; mention `locale/<lang>` folders in 4/03 |

## 6. Hand-over notes

- Content order: World 1 (pure Lua, no mock) → mock + World 2 → World 3 → World 4 → World 5.
- The mock and harness (`oxide/test.lua`, `mock_factorio.lua`) are engine work, not content: build them
  first with their own Lua self-tests, then content sessions only write quests.
- Content-check for the `lua` runner: starter must run (tests fail by `error("TODO")`, not by syntax
  errors), solution must be fully green, objective names must exist in `tests.lua` output.
