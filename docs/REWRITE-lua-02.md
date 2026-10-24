# Lua world 02 (factorio-data) rewrite

11 quests became 18 (3 reads, 14 exercises, the boss), each one idea, 3-7 minutes, in the golden shape
(unlabelled intro, kicker cards, one code block, Your turn, two hints, points that ask what the card
showed). No flavours: the whole world is the Factorio "Kit Mod". All `**Why now:**` labels and
field-reference bullet lists are gone. Term ledger rows: see the parent's merge into
`docs/tracks/lua-terms.md`.

| old quest | new quests |
|---|---|
| 01-setup (read) | 01-how-factorio-is-built (new: items, entities, recipes, technologies, science packs, labs), 02-how-a-mod-loads (prototype, data stage, `data.raw`, why it is frozen, control stage, vanilla, what Oxide fakes) |
| (new bridge) | 03-a-table-every-file-sees: globals, why world 01 avoided them, `data` as the one Factorio hands you |
| 02-info-json | 04-json-is-not-lua (JSON vs Lua table, `[[ ]]` long string), 05-the-mods-id-card (fields, dependency grammar, load order) |
| 03-first-item | 06-your-first-item (`data:extend`, array, type/name/icon/stack_size), 08-where-it-shows (item group, subgroup, order) |
| 10-reading-the-prototype-docs | 07-reading-the-prototype-docs (moved from just before the boss to right after the first item) |
| 04-icons-and-paths | 09-paths-start-with-a-root, 10-layered-icons |
| 05-recipes | 11-a-recipe: teaches the 2.0 `{type, name, amount}` shape straight from an empty list; the old shape is one "old tutorials" card |
| 06-copy-and-tweak | 12-copy-and-tweak (links back to 01/16 and 01/17) |
| 07-technologies | 13-locked-behind-research (`enabled`, `effects`, `prerequisites`), 14-the-research-bill (`unit`; says research is the *one* place the short `{name, amount}` pair is still right, which fixes the 05/07 contradiction) |
| 08-stage-order (halve vanilla stacks) | 15-wait-for-the-other-mod: reworked for the Kit Mod. A stand-in add-on `kit-extras` (depends on kit-mod) adds `kit-spring`; the kit-parts loop must move from `data.lua` to `data-updates.lua`. Load order (dependencies, then name) explained |
| 09-first-entity | 16-item-and-entity (place_result, minable, container, flags), 17-how-much-room-it-takes (tiles, y down, why 0.35, selection box) |
| 11-boss-new-item-chain | 18-boss-new-item-chain (same tests and starter; lesson in cards; points moved; says `kit-storage` needs its own icon) |

## Cut or changed

- `count_formula`, `research_trigger`: one "for the curious" card in 14, not used.
- `category`, `main_product`: dropped (the Kit Mod does not need them).
- Halving vanilla stack sizes (old 08): cut; it did not serve the mod.
- `pairs` order sentence from old 01: cut (world 01/15 already says "no promised order").
- Old 01 `requires = [] # TMP` was already fixed; 01's `requires = ["01-lua-core/23-catch-the-error"]` is kept.

## Ids

- Boss renumbered `11-boss-new-item-chain` -> `18-boss-new-item-chain` (slug kept) so it stays last.
- No `requires` anywhere in `content/` named a world-02 quest, so nothing else changed. `docs/tracks/lua.md`
  (a plan, not content) still says world 3 requires `02-factorio-data/07-technologies`; when world 3 is
  written, point it at `02-factorio-data/18-boss-new-item-chain` (or 14-the-research-bill).
- 12-copy-and-tweak links in-app to `lua/01-lua-core/16-two-names-one-table` and `17-copy-inside-copy`.
