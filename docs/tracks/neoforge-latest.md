# Track plan: `neoforge-latest`, NeoForge for the current Minecraft (delta track)

Planning doc for `content/tracks/neoforge-latest` (runner `gradle`, `jdk = 25`, `requires neoforge/...`).
A **delta** over [neoforge.md](neoforge.md): that track (Minecraft 1.21.1, JDK 21) stays exactly as planned
and is never edited by this one. See docs/TRACKS.md "Versions". Format rules: docs/CONTENT.md
(project first: every rewritten quest adds the **same project feature** as its 1.21.1 twin, with new code).

Researched **2026-10-19**. Sources: Maven metadata (maven.neoforged.net, Gradle plugin portal), Mojang
version manifest, the MDK for 26.3, NeoForge docs (version 26.1 shown as current), NeoForge primers
(`neoforged/.github/primers`), Modrinth version lists. Items marked **verify** were not confirmed in a
primary source; the measurement spike (neoforge.md section 1) resolves them before content is written.
Timings and XP are **estimates**, same as neoforge.md.

## 0. Pins at a glance

| Thing | Pin (2026-10-19) |
|---|---|
| Minecraft | **26.3** (released 2026-09-15; Mojang moved to year-based names: 26.1 = first 2026 drop) |
| NeoForge | **26.3.0.94** (newest stable in the `26.3` line; the MDK pins 26.3.0.90; freeze one at content-writing time). Format is now `<mc>.<patch>.<build>`: first three parts = Minecraft version |
| JDK | **25** (required to build since 26.1; Mojang ships Java 25 to players from 26.3) |
| Build plugin | ModDevGradle `net.neoforged.moddev` **2.0.148** (MDK 26.3 uses it; 2.0.141 failed to recompile 26.3 per a mod author, 2.0.147+ ok) |
| Gradle | **9.2.1** (MDK wrapper; 26.1 post says 9.1.0 minimum) |
| Mappings | Mojang official names; vanilla is **unobfuscated** since 26.1, so Parchment is optional (MDK 26.3 has no Parchment line) |
| Architectury API | **22.0.5+neoforge** for 26.3 (2026-10-09). Architectury Loom for 26.x: pin from the template at writing time, **verify** |
| Next | Minecraft 26.4 snapshots are out (26.4-snapshot-4, 2026-10-13): expect 26.4 around December |

Cadence: 26.1 (NeoForge post 2026-03-24), 26.2 (~June), 26.3 (2026-09-15). **One Minecraft release per ~3
months**, each with API breaks (the 26.2 and 26.3 primers are each hundreds of KB). See risks.

## 1. Toolchain and workspace

### What differs from the 1.21.1 workspace

| Area | 1.21.1 (neoforge.md) | `neoforge-latest` |
|---|---|---|
| Workspace dir | `workspaces/neoforge/<world>/` | `workspaces/neoforge-latest/<world>/`, never shares files with the 1.21.1 one |
| `JAVA_HOME` | JDK 21 | JDK **25** (`track.toml jdk = 25`; Setup quest installs Temurin 25 via winget after asking, TRACKS.md) |
| `build.gradle` | MDG 2.0.148, toolchain 21 | MDG 2.0.148, `java.toolchain.languageVersion = JavaLanguageVersion.of(25)`; engine-owned, same for every world of this track |
| Wrapper | Gradle 8.x | Gradle **9.2.1** |
| `gradle.properties` | `minecraft_version`, `neo_form_version`/`neo_version`, Parchment | `minecraft_version=26.3`, `neo_version=26.3.0.94`, **no Parchment**; version range `[26.3]` |
| `neoforge.mods.toml` | dependency range for 1.21.1 | range for 26.3 (four-part NeoForge versions); loader range unchanged **verify** |
| Run types | `client`, `server`, `gameTestServer`, `data`, `unitTest` | `client`, `server`, `gameTestServer`, `data` in the MDK; `unitTest` DSL block still to **verify** |
| GameTest enabling | `-Dneoforge.enabledGameTestNamespaces=<mod_id>` | docs say `neoforge.enableGameTest=true` for non-gametest run types; MDG's `gameTestServer` run enables the mod namespace itself. **verify** which property the reporter hook needs |
| Datagen output | `src/generated/resources` | same; plus test instances and client item definitions are generated (see worlds 00, 02) |
| Memory | `-Xmx3G` in the reference mod | MDK ships `-Xmx1G`; keep 3G for decompile. Do **not** copy `org.gradle.daemon=false` (same rule as neoforge.md) |

### Starter model

Unchanged topics are not repeated as quests, but the workspace of a world must still compile and contain
the contract classes the unchanged quests would have created (`ModItems.RUBY`, `LedgerData`, ...).
Quest 01 of each world therefore copies a **ported baseline** (the 1.21.1 reference solution of all
earlier unchanged quests, already ported to 26.3 by the content author) next to the normal starter.
Learner code from the 1.21.1 workspace is never imported.

### First run, time, size (estimates)

- First ever run with a fresh `GRADLE_USER_HOME` for this pin: **3-8 min**, ~2 GB (Gradle 9.2.1 dist, JDK
  25 if not installed, Minecraft 26.3 client/server jars, assets, NeoForm run). Deobfuscation removes the
  remap step, so maybe 20-30% less work than 1.21.1; **measure**.
- Second workspace: 20-40 s; ~0.5 GB per active world workspace.
- Shared cache is **not** shared with the 1.21.1 track (different Minecraft jars, different Gradle).
  A learner doing both tracks pays ~2.4 GB twice. Setup quest shows both numbers.

### Rule: never overwrite 1.21.1 content

1. Track id is `neoforge-latest`; quest ids `neoforge-latest/<world>/<NN-slug>` (world folders reuse the
   1.21.1 names: `00-setup`, ..., `11-capstone`).
2. Files live only in `content/tracks/neoforge-latest/`. No file in `content/tracks/neoforge/` is edited,
   renamed or deleted; content-check fails if a `neoforge-latest` quest id collides with a `neoforge` one.
3. Progress is per quest id: finishing a latest quest never marks its 1.21.1 twin done, and vice versa.
4. `requires` uses three-segment ids into `neoforge/...` (CONTENT.md). Consequence: **this track is gated
   behind the 1.21.1 one** by default. Open question 1.

## 2. Worlds

Kind: `ex` exercise, `read` quiz quest, `boss`. `ex·client` = lesson sends the learner to `runClient`,
graded objectives still headless. Column **Replaces / requires** names the 1.21.1 quest
(`neoforge/<world>/<quest>`) the quest replaces; a rewritten quest `requires` that 1.21.1 quest and
the previous quest of this track. **Landed** = Minecraft version where the change landed (NeoForge line
in brackets when it differs). `unchanged:` lines are **not quests**: the learner's 1.21.1 completion
counts, the quest does not appear in this track.

| World | Title | Quests | XP | Minutes |
|---|---|---|---|---|
| 00 | Setup | 5 | 425 | 61 |
| 01 | Registries | 7 | 590 | 93 |
| 02 | Data and datagen | 9 | 670 | 102 |
| 03 | Events and attachments | 6 | 555 | 87 |
| 04 | Interaction and menus | 5 | 570 | 84 |
| 05 | Networking | 6 | 615 | 85 |
| 06 | Entities | 6 | 665 | 91 |
| 07 | World | 3 | 475 | 61 |
| 08 | Commands and config | 3 | 470 | 54 |
| 09 | Architectury (multi-loader) | 5 | 650 | 98 |
| 10 | Ecosystem libraries | 7 | 770 | 122 |
| 11 | Capstone: port | 7 | 805 | 154 |
| | **Total** | **69** | **7260** | **1092** (~18.2 h) |

(1.21.1 track: 116 quests, 9550 XP. The 47 quests not repeated here are the `unchanged:` lines.)

Every world opens with a `what-moved` read quest: 15 XP, 4 min, quiz 1-3 questions, `requires` the
world's first 1.21.1 quest. Its objective is `quiz`. Cheat-sheet rows (section 3) feed the questions.

### World 00: Setup (`00-setup`, 5 quests, 425 XP)

JDK 25, four-part versions, official names, GameTest registration.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | JDK 25, Gradle 9, `26.3.0.x` versions, no obfuscation, quarterly releases, where primers live. | quiz | 4 | 15 | read | requires `neoforge/00-setup/01-jdk-and-gradle` | 26.1 |
| 02 | `pins-26-3` | Set toolchain, wrapper, `neo_version`, version range in `gradle.properties` and `neoforge.mods.toml`. | `java_toolchain_is_25`<br>`wrapper_is_gradle_9_or_newer`<br>`neo_version_matches_minecraft`<br>`dependency_range_covers_26_3` | 8 | 40 | ex | replaces `neoforge/00-setup/03-mods-toml` | 26.1 |
| 03 | `official-names` | Vanilla is unobfuscated; Parchment optional; reading decompiled vanilla now. | quiz | 4 | 15 | read | replaces `neoforge/00-setup/07-parchment-names` | 26.1 |
| 04 | `gametest-registration` | Register a test function in `BuiltInRegistries.TEST_FUNCTION`, add a test instance (JSON or `RegisterGameTestsEvent`), run `runGameTestServer`. | `test_function_registered`<br>`test_instance_json_valid`<br>`gametest_server_runs_it` | 10 | 55 | ex | new (1.21.1 teaches GameTests only inside bosses); requires `neoforge/00-setup/08-boss-welcome-mod` | 1.21.5 |
| 05 | `boss-welcome-mod` | Port the welcome mod: join greeting and a visit counter that survives restarts. | `greeting_formats_player_name`<br>`visit_counter_increments`<br>`greeting_sent_on_join`<br>`mod_loads_in_gametest_server` | 35 | 300 | **boss** | replaces `neoforge/00-setup/08-boss-welcome-mod` | 1.21.5, 1.21.6, 1.21.11, 26.1 |

unchanged: requires `neoforge/00-setup/02-name-your-mod`<br>
unchanged: requires `neoforge/00-setup/04-hello-mod`<br>
unchanged: requires `neoforge/00-setup/05-run-the-client`<br>
unchanged: requires `neoforge/00-setup/06-setup-listener`

(Boss 05 touches `ResourceLocation` -> `Identifier`, saved-data ids (26.1) and value-IO persistence.)

### World 01: Registries (`01-registries`, 7 quests, 590 XP)

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | Ids live in `Properties`, `InteractionResult` unified, behaviour as data components, value-IO, `ItemStackTemplate`. | quiz | 4 | 15 | read | requires `neoforge/01-registries/01-deferred-register` | 1.21.2 to 26.1 |
| 02 | `first-item-ids` | Item via `registerSimpleItem`; `setId` is mandatory; stack size 16. | `item_registered_with_id`<br>`item_stack_size_is_16`<br>`properties_have_id` | 8 | 40 | ex | replaces `neoforge/01-registries/02-first-item` | 1.21.2 |
| 03 | `food-consumable` | Food + `Consumable` component, `ConsumeEffect`, use remainder. | `food_nutrition_set`<br>`consumable_has_eat_effect`<br>`use_remainder_set` | 8 | 45 | ex | replaces `neoforge/01-registries/03-food-item` | 1.21.2 |
| 04 | `first-block-ids` | Block with `setId`, `registerSimpleBlockItem`, description prefix, hardness, sound. | `block_registered`<br>`block_item_registered`<br>`hardness_and_sound_set`<br>`block_properties_have_id` | 9 | 45 | ex | replaces `neoforge/01-registries/05-first-block` | 1.21.2 |
| 05 | `block-entity-valueio` | Public `BlockEntityType` ctor (builder gone), `saveAdditional(ValueOutput)` / `loadAdditional(ValueInput)` round-trip. | `be_type_registered`<br>`be_saves_and_loads_value_io`<br>`be_valid_blocks_set` | 12 | 65 | ex | replaces `neoforge/01-registries/07-block-entity` | 1.21.2 (type), 1.21.6 (value-IO) |
| 06 | `data-components-26` | Custom `DataComponentType` with codec; stack stores charge; `ItemStackTemplate` builds a stack from a data file. | `component_registered`<br>`component_codec_roundtrips`<br>`stack_stores_component`<br>`template_creates_stack_with_component` | 12 | 60 | ex | replaces `neoforge/01-registries/09-data-components` | 1.21.5 (`DataComponentGetter`), 26.1 (`ItemStackTemplate`) |
| 07 | `boss-pocket-lantern` | Port the glowing, burning-down lantern block and its rechargeable item. | `lantern_block_registered`<br>`lantern_light_level_toggles`<br>`charge_component_roundtrips`<br>`burn_time_persists`<br>`placed_lantern_emits_light` | 40 | 320 | **boss** | replaces `neoforge/01-registries/10-boss-pocket-lantern` | 1.21.2, 1.21.6, 26.1 |

unchanged: requires `neoforge/01-registries/04-creative-tab` (`BuildCreativeModeTabContentsEvent`, `CreativeModeTab.builder()` per 26.1 docs)<br>
unchanged: requires `neoforge/01-registries/06-blockstate-property`<br>
unchanged: requires `neoforge/01-registries/08-ticking-be` (**verify** ticker signature)

### World 02: Data and datagen (`02-data`, 9 quests, 670 XP)

Biggest churn after rendering: models became client item definitions, recipes became a registry, loot moved to reloadable registries.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | `items/` definitions, recipe runner, `ItemStackTemplate` results, loot types, reloadable registries. | quiz | 4 | 15 | read | requires `neoforge/02-data/01-datagen-wiring` | 1.21.2 to 26.3 |
| 02 | `datagen-events` | Gather-data handler with client and server provider sets; `runData` writes `src/generated`. | `providers_registered`<br>`generated_dir_configured`<br>`client_and_server_providers_split` | 8 | 40 | ex | replaces `neoforge/02-data/01-datagen-wiring` | 1.21.4 to 26.x (**verify** split event names `GatherDataEvent.Client/.Server`) |
| 03 | `item-definitions` | Generate `items/<id>.json` client item definition and its model. | `item_definition_exists`<br>`item_definition_points_to_model`<br>`item_model_parent_correct` | 8 | 50 | ex | replaces `neoforge/02-data/03-item-models` | 1.21.4 (**verify** in 1.21.4 primer); `ITEM_MODEL` component 1.21.2 |
| 04 | `block-models-26` | Blockstate, block model and **item definition** for a block (`BlockModelGenerators` signature churn). | `blockstate_has_variants`<br>`block_model_texture_set`<br>`block_item_definition_exists` | 10 | 55 | ex | replaces `neoforge/02-data/04-block-models` | 1.21.4, 26.2 |
| 05 | `tags-provider` | Block and item tags with the reworked tag providers. | `block_in_mineable_tag`<br>`tool_tier_tag_set` | 4 | 30 | ex | replaces `neoforge/02-data/05-tags` | 1.21.6 |
| 06 | `recipes-26` | Shaped, shapeless, smelting; recipe provider + runner, `ItemStackTemplate` result, unlock advancement; recipes now a reloadable registry. | `shaped_recipe_json`<br>`shapeless_recipe_json`<br>`recipe_has_unlock_criterion`<br>`result_uses_item_stack_template` | 10 | 60 | ex | replaces `neoforge/02-data/06-recipes` | 1.21.2 (runner), 26.1 (`ItemStackTemplate`, serializer record), 26.3 (reloadable) |
| 07 | `loot-tables-26` | Block drops itself; silk-touch-only; `LootTableSubProvider.Factory`, `Holder<LootItemCondition>`. | `block_drops_itself`<br>`silk_touch_only_drop` | 10 | 55 | ex | replaces `neoforge/02-data/07-loot-tables` | 26.1 (types unrolled), 26.3 (reloadable, provider API) |
| 08 | `reloadable-registries` | Which registries reload with `/reload` (loot, recipes, advancements) and what `Holder` wrappers mean. | quiz | 3 | 15 | read | replaces `neoforge/02-data/09-handwritten-vs-generated` | 26.3 |
| 09 | `boss-ruby-pack` | Port the full ruby content set from one deterministic datagen run. | `datagen_is_deterministic`<br>`all_registered_objects_have_models`<br>`all_blocks_have_loot_tables`<br>`recipes_unlock_correctly`<br>`lang_has_no_missing_keys` | 45 | 350 | **boss** (client) | replaces `neoforge/02-data/10-boss-ruby-pack` | 1.21.4 to 26.3 |

unchanged: requires `neoforge/02-data/02-lang-provider` (**verify**)<br>
unchanged: requires `neoforge/02-data/08-data-maps` (**verify**)

### World 03: Events and attachments (`03-events`, 6 quests, 555 XP)

Events are stable. Attachments change serialisation; item/energy transfer was redesigned.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | Events stable; value-IO for attachments; transfer API replaced; item attachments superseded by components. | quiz | 4 | 15 | read | requires `neoforge/03-events/01-event-bus-tour` | 1.21.5 to 1.21.9 |
| 02 | `attachments-valueio` | Player `AttachmentType` with map codec or `ValueIOSerializable`, default, sync option. | `attachment_registered`<br>`attachment_codec_roundtrip`<br>`attachment_has_default` | 10 | 55 | ex | replaces `neoforge/03-events/06-attachments` | 1.21.6 (**verify** `serialize(MapCodec)` vs 1.21.1 `Codec`) |
| 03 | `transaction-model` | Resources (`ItemResource`) vs amounts, `ResourceHandler`, `Transaction` commit/rollback. | quiz | 4 | 15 | read | new; requires `neoforge/03-events/08-item-capability` | 1.21.9 (NeoForge 21.9) |
| 04 | `item-resource-handler` | `ResourceHandler<ItemResource>` on a stack via `Capabilities.Item.ITEM`; insert and extract in a transaction. | `item_handler_capability_registered`<br>`insert_and_extract_work`<br>`failed_transaction_rolls_back` | 12 | 60 | ex | replaces `neoforge/03-events/08-item-capability` | 1.21.9 |
| 05 | `energy-handler` | `EnergyHandler` on a block entity, neighbour extracts inside a transaction. | `energy_capability_registered`<br>`neighbour_can_extract`<br>`extract_respects_transaction` | 12 | 60 | ex | replaces `neoforge/03-events/09-block-capability` | 1.21.9 |
| 06 | `boss-miners-ledger` | Port the ledger: blocks-mined counter that survives death and relog. | `counter_increments_on_break`<br>`counter_survives_death`<br>`counter_survives_relog`<br>`ledger_reports_count`<br>`capability_exposes_ledger_slot` | 45 | 350 | **boss** | replaces `neoforge/03-events/10-boss-miners-ledger` | 1.21.6, 1.21.9 |

unchanged: requires `neoforge/03-events/02-first-subscriber`, `03-cancel-and-priority`, `04-incoming-damage`, `05-block-break-event`, `07-copy-on-death` (each **verify** event class names after a compile spike)<br>
Cross-world: boss shape is shared with the Fabric and Architectury capstones; keep `LedgerData` public shape stable.

### World 04: Interaction and menus (`04-interaction`, 5 quests, 570 XP)

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | One `InteractionResult`, use cooldown as component, GUI rendering via render state. | quiz | 4 | 15 | read | requires `neoforge/04-interaction/01-use-item` | 1.21.2, 1.21.6, 26.1 |
| 02 | `use-item-result` | `Item.use` returns `InteractionResult`; cooldown from the `USE_COOLDOWN` component. | `use_returns_success_server_side`<br>`cooldown_applied` | 8 | 45 | ex | replaces `neoforge/04-interaction/01-use-item` | 1.21.2 |
| 03 | `block-use-result` | Lever-like block: `useItemOn` may return `TRY_WITH_EMPTY_HAND`; `useWithoutItem` toggles. | `empty_hand_toggles`<br>`item_hand_passes_through` | 10 | 50 | ex | replaces `neoforge/04-interaction/05-block-use` | 1.21.2 |
| 04 | `screen-extract` | `AbstractContainerScreen` through `extractRenderState` / `GuiGraphicsExtractor`, still registered through the menu-screens event. | `screen_registered_for_menu`<br>`gui_texture_path_exists`<br>`screen_overrides_extract_methods` | 12 | 60 | ex·client | replaces `neoforge/04-interaction/07-screen-class` | 1.21.6 (`GuiRenderState`), 26.1 (`GuiGraphicsExtractor`, **verify** exact overrides) |
| 05 | `boss-sorting-chest` | Port the sorting chest: menu, merge-and-sort, persistence, hopper insert. | `menu_opens_on_right_click`<br>`quick_move_respects_slots`<br>`sort_merges_stacks`<br>`contents_persist_in_world`<br>`hopper_can_insert` | 50 | 400 | **boss** | replaces `neoforge/04-interaction/10-boss-sorting-chest` | 1.21.6 (container value-IO), 1.21.9 (transfer) |

unchanged: requires `neoforge/04-interaction/02-use-on-block`, `03-interact-entity`, `04-sound-and-particles`, `06-menu-and-slots`, `08-look-check-screen`, `09-container-data` (each **verify**)

### World 05: Networking (`05-networking`, 6 quests, 615 XP)

The payload model survived. Names, client-only handlers and key mappings moved.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | `Identifier`, client-only handler event, `ClientPacketDistributor`, key categories, SDL input. | quiz | 4 | 15 | read | requires `neoforge/05-networking/01-payload-model` | 1.21.9 to 26.3 |
| 02 | `identifier-payload` | Payload record, `Type`, codec, id built with `Identifier`. | `payload_codec_roundtrip`<br>`payload_type_id_matches_mod` | 6 | 35 | ex | replaces `neoforge/05-networking/03-first-payload` | 1.21.11 |
| 03 | `client-handlers` | `RegisterPayloadHandlersEvent` plus `RegisterClientPayloadHandlersEvent` so server jars never load client handler classes. | `registrar_version_set`<br>`both_directions_registered`<br>`client_handler_registered_separately` | 10 | 55 | ex | replaces `neoforge/05-networking/04-register-payloads` | 21.x (**verify** line) |
| 04 | `send-to-server` | Client sends with `ClientPacketDistributor`; server handler validates distance and permission on the main thread. | `rejects_far_players`<br>`accepts_valid_request`<br>`handler_runs_on_main_thread` | 10 | 55 | ex | replaces `neoforge/05-networking/05-client-to-server` | 21.x (**verify**), 1.21.11 (permissions) |
| 05 | `keymapping-category` | `KeyMapping` with a `KeyMapping.Category` and SDL key constants; press sends a payload. | `keymapping_registered`<br>`category_registered`<br>`press_sends_payload` | 10 | 55 | ex·client | replaces `neoforge/05-networking/08-keybind-trigger` | 1.21.9 (**verify** category), 26.3 (SDL, `InputConstants`) |
| 06 | `boss-ping-wand` | Port the ping wand: key press, server validation, nearby marker particle. | `payload_roundtrips`<br>`server_rejects_spam`<br>`marker_reaches_nearby_players`<br>`no_client_class_on_server`<br>`cooldown_enforced_server_side` | 45 | 400 | **boss** (client) | replaces `neoforge/05-networking/10-boss-ping-wand` | 1.21.9 to 26.3 |

unchanged: requires `neoforge/05-networking/02-stream-codecs`, `06-server-to-client`, `07-sync-attachment`, `09-trust-nothing` (each **verify**)

### World 06: Entities (`06-entities`, 6 quests, 665 XP)

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | Entity type needs a key, render states, value-IO for entities, submit-node rendering. | quiz | 4 | 15 | read | requires `neoforge/06-entities/01-entity-types` | 1.21.2, 1.21.6, 1.21.9 |
| 02 | `entity-type-key` | Register type with `EntityType.Builder#build(ResourceKey)` and attributes. | `entity_type_registered`<br>`attributes_registered`<br>`entity_spawns_in_world` | 10 | 55 | ex | replaces `neoforge/06-entities/02-register-entity` | 1.21.2 |
| 03 | `spawn-egg-26` | Spawn egg as data on `Item.Properties`, spawn placement. | `spawn_egg_registered`<br>`spawn_placement_rules_set` | 5 | 30 | ex | replaces `neoforge/06-entities/04-spawn-egg` | 1.21.5 to 26.1 (**verify** exact) |
| 04 | `entity-render-state` | `EntityRenderState` extraction, model generic over state, layer definition and renderer registration. | `renderer_registered`<br>`layer_definition_registered`<br>`render_state_extracts_fields` | 12 | 60 | ex·client | replaces `neoforge/06-entities/07-renderer` | 1.21.2 (states), 1.21.9 (`submit`) |
| 05 | `entity-save-valueio` | Persist the nest position with `addAdditionalSaveData(ValueOutput)` / `readAdditionalSaveData(ValueInput)`. | `nest_pos_saved`<br>`nest_pos_loaded_with_default` | 10 | 55 | ex | new; requires `neoforge/06-entities/10-boss-magpie` data model | 1.21.6 |
| 06 | `boss-magpie` | Port the magpie: pick up items, deliver to nest, flee when hurt, persist. | `magpie_spawns`<br>`picks_up_item`<br>`delivers_to_nest`<br>`ignores_when_hurt`<br>`persists_across_reload` | 50 | 450 | **boss** (client) | replaces `neoforge/06-entities/10-boss-magpie` | 1.21.2 to 1.21.9 |

unchanged: requires `neoforge/06-entities/03-attribute-tuning`, `05-goal-basics`, `06-custom-goal`, `08-look-check-renderer` (each **verify**); `09-entity-loot` JSON is unchanged but its datagen follows `02-data/07-loot-tables-26`.

Order inside the world: 01, 02, 03, 05 (needs 02), 04, 06.

### World 07: World (`07-world`, 3 quests, 475 XP)

Little changed for mod authors. Honest delta: one real rewrite, one boss.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | World clocks, `Level#getRandom()`, saved-data ids, reloadable worldgen-adjacent registries; structure NBT must be re-saved or datafixed. | quiz | 4 | 15 | read | requires `neoforge/07-world/01-worldgen-json` | 26.1 |
| 02 | `dimension-type-clock` | Dimension type with a default `WorldClock`, dragon-fight flag, level stem. | `dimension_type_json_valid`<br>`level_stem_references_type`<br>`dimension_type_references_clock` | 12 | 60 | ex | replaces `neoforge/07-world/08-dimension-type` | 26.1 |
| 03 | `boss-sunken-ruin` | Port the ruin: ore, structure, loot chest, `/locate`. | `ore_generates_in_test_chunk`<br>`template_places_loot_chest`<br>`structure_set_valid`<br>`biome_modifier_valid`<br>`loot_chest_has_contents` | 45 | 400 | **boss** | replaces `neoforge/07-world/09-boss-sunken-ruin` | 26.1, 26.3 |

unchanged: requires `neoforge/07-world/01-worldgen-json`, `02-ore-feature`, `03-biome-modifier`, `04-vein-tuning`, `05-structure-template`, `06-structure-set`, `07-dimension-intro` (each **verify**; the data pack format number moved, see risks).

### World 08: Commands and config (`08-commands-config`, 3 quests, 470 XP)

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | Permission levels replaced by `Permission` / `PermissionCheck`; vanilla now has `/waypoint`. | quiz | 4 | 15 | read | requires `neoforge/08-commands-config/01-brigadier-literals` | 1.21.11 |
| 02 | `permission-checks` | `requires(...)` with a `PermissionCheck`, suggestion provider, non-op denied. | `requires_permission_check`<br>`suggestions_listed`<br>`non_op_denied` | 10 | 55 | ex | replaces `neoforge/08-commands-config/04-permissions-suggestions` | 1.21.11 |
| 03 | `boss-waypoints` | Port `/waypoint set\|tp\|list`; rename the root (vanilla `/waypoint` exists since 1.21.6, **verify**), limit from config, attachment storage. | `set_stores_waypoint`<br>`tp_teleports_player`<br>`limit_from_config_enforced`<br>`list_output_sorted`<br>`waypoints_persist` | 40 | 400 | **boss** | replaces `neoforge/08-commands-config/10-boss-waypoints` | 1.21.6, 1.21.11 |

unchanged: requires `neoforge/08-commands-config/01-brigadier-literals`, `02-brigadier-arguments`, `03-command-feedback`, `05-register-commands`, `06-config-spec`, `07-config-reload`, `08-builtin-config-screen`, `09-cloth-config-screen` (Cloth Config **26.3.159+neoforge** exists; each **verify** by compile)

### World 09: Architectury (`09-architectury`, 5 quests, 650 XP)

Architectury API has a 26.3 build (22.0.5). The Fabric half needs a non-remapping Loom on JDK 25.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | Architectury API 22, JDK 25 in all modules, no remap step, what stays `@ExpectPlatform`. | quiz | 4 | 15 | read | requires `neoforge/09-architectury/01-multiloader-map` | 26.1 |
| 02 | `loom-project-26` | Three-module project on JDK 25, shared `mod_id`. | `project_has_three_modules`<br>`mod_id_shared_across_modules`<br>`toolchain_is_25_in_all_modules` | 8 | 45 | ex | replaces `neoforge/09-architectury/02-loom-project` | 26.1 (**verify** Loom plugin ids) |
| 03 | `arch-registries-26` | Architectury `DeferredRegister` item for both loaders, with `setId`. | `item_registered_via_architectury`<br>`registrar_uses_mod_id`<br>`item_properties_have_id` | 12 | 70 | ex | replaces `neoforge/09-architectury/06-arch-registries` | 1.21.2 |
| 04 | `arch-networking-26` | `NetworkManager` payload from common code, `Identifier` ids. | `payload_registered_common`<br>`handler_runs_on_both_loaders`<br>`payload_id_is_identifier` | 14 | 70 | ex | replaces `neoforge/09-architectury/08-arch-networking` | 1.21.11 |
| 05 | `boss-ledger-everywhere` | Port the ledger to common code on both loaders. | `common_logic_tests_pass`<br>`neoforge_gametest_passes`<br>`registry_via_architectury`<br>`packet_in_common`<br>`both_jars_have_metadata` | 60 | 450 | **boss** (client) | replaces `neoforge/09-architectury/11-boss-ledger-everywhere`; requires `neoforge-latest/03-events/06-boss-miners-ledger` | 1.21.2 to 26.1 |

unchanged: requires `neoforge/09-architectury/03-common-code`, `04-expect-platform`, `05-platform-detect`, `07-arch-events`, `09-service-loader`, `10-build-both-jars` (each **verify**; `07-arch-events` result types)

### World 10: Ecosystem libraries (`10-ecosystem`, 7 quests, 770 XP)

Status of each library on 26.3 NeoForge (Modrinth, 2026-10-19):

| Library | 26.3 NeoForge build | Plan |
|---|---|---|
| Cloth Config | 26.3.159 | covered in world 08 as unchanged |
| JEI | 31.16.0.85 | quest 02 |
| Curios | 17.0.0-beta.3+26.3 (beta) | quest 03 |
| GeckoLib | 5.5.7 (new major, 4.x gone) | quests 04, 05 |
| Jade | 26.3.5+neoforge | quest 06 |
| EMI | none found on Modrinth (latest there 1.1.24 for 1.21.1) | blocked, **verify** on EMI's own maven/CurseForge |
| Accessories | none for 26.x NeoForge (1.21.1 only) | not taught either way |
| KubeJS | 26.1.2-8.0.7+neoforge (no 26.3) | blocked until a 26.3 build |
| owo-lib | NeoForge 1.21.1 only; 0.13.1 for 26.2 is Fabric only | blocked until a NeoForge 26.x build |

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `what-moved` | Which libraries are on 26.3, which lag, what "blocked" quests do. | quiz | 5 | 15 | read | requires `neoforge/10-ecosystem/01-dependency-map` | 26.3 |
| 02 | `jei-plugin-31` | `IModPlugin`, recipe category with `Identifier` ids, JEI 31 API. | `plugin_uid_matches_mod`<br>`recipe_category_registered` | 12 | 60 | ex | replaces `neoforge/10-ecosystem/03-jei-plugin` | JEI 31 (**verify** API) |
| 03 | `curios-slot-17` | Charm slot JSON + tag, equippable item, Curios 17 API. | `slot_json_exists`<br>`item_equippable_in_slot` | 12 | 65 | ex | replaces `neoforge/10-ecosystem/05-curios-slot` | Curios 17 beta (**verify** API) |
| 04 | `geckolib-5-model` | GeckoLib 5 model, texture and animation paths, render-state based. | `geo_paths_exist`<br>`animatable_registers_controller` | 12 | 65 | ex·client | replaces `neoforge/10-ecosystem/06-geckolib-model` | GeckoLib 5 (**verify** API) |
| 05 | `geckolib-5-animation` | Controller and trigger; look-check at the end. | `controller_plays_idle`<br>`trigger_animation_fires` | 12 | 60 | ex·client | replaces `neoforge/10-ecosystem/07-geckolib-animation` | GeckoLib 5 |
| 06 | `jade-tooltip-26` | Jade plugin shows block entity state via the server data provider. | `jade_plugin_registers_provider`<br>`tooltip_text_contains_state` | 10 | 55 | ex | replaces `neoforge/10-ecosystem/08-jade-tooltip` | Jade 26.x |
| 07 | `boss-compat-pack` | Port the compat pack: JEI, Jade, GeckoLib, Curios, still runs without them. | `jei_recipes_registered`<br>`jade_provider_registered`<br>`curio_slot_registered`<br>`runs_without_optional_mods` | 60 | 450 | **boss** (client) | replaces `neoforge/10-ecosystem/11-boss-compat-pack` | 26.3 |

unchanged: requires `neoforge/10-ecosystem/02-soft-dependencies` (**verify**)<br>
blocked (listed, no quest until a build exists): `neoforge/10-ecosystem/04-emi-plugin`, `09-kubejs-hook`, `10-owo-screen`. When one ships, add a quest and extend the boss with its objective (`emi_recipes_registered` and so on).

### World 11: Capstone, port the 1.21.1 mod (`11-capstone`, 7 quests, 805 XP)

Port the learner's **own** 1.21.1 capstone (their `neoforge/11-capstone` workspace) to 26.3. The learner
copies it in; if they never did the 1.21.1 capstone they port the reference solution.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Replaces / requires | Landed |
|---|---|---|---|---|---|---|---|---|
| 01 | `port-plan` | Audit the old mod against the cheat sheet, list what breaks, order the work. | quiz | 4 | 15 | read | requires `neoforge/11-capstone/05-metadata-polish` | 1.21.2 to 26.3 |
| 02 | `bump-toolchain` | Pins, toolchain 25, metadata and version ranges for 26.3. | `java_toolchain_is_25`<br>`neo_version_matches_minecraft`<br>`three_modules_present`<br>`metadata_has_license_and_links` | 10 | 50 | ex | replaces `neoforge/11-capstone/02-mod-skeleton` | 26.1 |
| 03 | `port-registries-and-data` | Ids in properties, item definitions, recipes and loot via the new providers. | `blocks_and_items_registered`<br>`item_definitions_generated`<br>`recipes_use_item_stack_template` | 15 | 70 | ex | replaces `neoforge/11-capstone/03-feature-common` | 1.21.2 to 26.3 |
| 04 | `port-logic-and-network` | Value-IO persistence, payloads with `Identifier`, permission checks, config. | `block_entity_roundtrips`<br>`payload_registered`<br>`command_permission_checked`<br>`config_spec_valid` | 15 | 70 | ex | replaces `neoforge/11-capstone/04-feature-loaders` | 1.21.6 to 1.21.11 |
| 05 | `port-gametests` | Move the GameTest suite to registered test functions and instances. | `gametests_registered`<br>`suite_discovered_by_server`<br>`gametest_server_exits_zero` | 12 | 60 | ex | new; requires `neoforge-latest/00-setup/04-gametest-registration` | 1.21.5 |
| 06 | `modrinth-26` | Minotaur targets game version 26.3 and the loaders. | `minotaur_configured`<br>`game_versions_include_26_3` | 8 | 40 | ex | replaces `neoforge/11-capstone/06-modrinth-config` | 26.3 |
| 07 | `boss-publish` | Ship it: both jars, GameTests green, Modrinth page live (manual last step). | `both_jars_build`<br>`gametest_suite_passes`<br>`feature_logic_covered`<br>`no_client_code_in_common`<br>`dependencies_declared`<br>`changelog_matches_version` | 90 | 500 | **boss** (client) | replaces `neoforge/11-capstone/07-boss-publish` | 26.3 |

## 3. Porting cheat sheet (1.21.1 to 26.3)

Primer links: `https://github.com/neoforged/.github/blob/main/primers/<version>/index.md`
(folders exist for 1.21.2, 1.21.4, 1.21.5, 1.21.6, 1.21.7-1.21.11, 26.1, 26.2, 26.3). Docs:
https://docs.neoforged.net (current = 26.1; 1.21.1 docs via the version menu).

| Topic | 1.21.1 way | Current (26.3) way | Changed in | Source |
|---|---|---|---|---|
| JDK / Gradle | JDK 21, Gradle 8.8+ | JDK 25, Gradle 9.1+ (MDK 9.2.1) | 26.1 | [26.1 release post](https://neoforged.net/news/26.1release/) |
| Mappings | Mojang + Parchment | Mojang official names, vanilla unobfuscated, Parchment optional | 26.1 | [26.1 primer](https://github.com/neoforged/.github/blob/main/primers/26.1/index.md), 26.1 post |
| NeoForge version | `21.1.x` | `26.3.0.<build>`, first three = MC version | 26.1 | 26.1 post |
| Resource ids | `ResourceLocation` | `Identifier` (`fromNamespaceAndPath`, `readIdentifier`, `ResourceKey#identifier`) | 1.21.11 | [1.21.11 primer](https://github.com/neoforged/.github/blob/main/primers/1.21.11/index.md) |
| Registered object ids | `new Item(props)` | `Item.Properties#setId(ResourceKey)` required; same for `BlockBehaviour.Properties`; `DeferredRegister.Items#registerSimpleItem` | 1.21.2 | [1.21.2 primer](https://github.com/neoforged/.github/blob/main/primers/1.21.2/index.md), [items docs](https://docs.neoforged.net/docs/items/) |
| Interaction results | `InteractionResultHolder`, `ItemInteractionResult` | one `InteractionResult` interface (`SUCCESS`, `TRY_WITH_EMPTY_HAND`, ...) | 1.21.2 | 1.21.2 primer |
| Item names and models | `models/item/*.json`, overrides | `DataComponents#ITEM_MODEL`, client item definitions `items/*.json` | 1.21.2 / 1.21.4 (**verify** in [1.21.4 primer](https://github.com/neoforged/.github/blob/main/primers/1.21.4/index.md)) | 1.21.2 primer |
| Item behaviour | `Tier`, `ArmorMaterial`, `ElytraItem`, `FoodProperties` only | `ToolMaterial`, `Equippable`, `Consumable`, `GLIDER`, `ENCHANTABLE`, `REPAIRABLE` components | 1.21.2 | 1.21.2 primer |
| Tools and weapons | `SwordItem`, `DiggerItem`, `ArmorItem` | `Tool`, `Weapon`, `BlocksAttacks` components; `TOOLTIP_DISPLAY` replaces hide flags | 1.21.5 | [1.21.5 primer](https://github.com/neoforged/.github/blob/main/primers/1.21.5/index.md) |
| New item components | n/a | `USE_EFFECTS`, `SWING_ANIMATION`, `KINETIC_WEAPON`, `PIERCING_WEAPON`, ... | 1.21.11 | 1.21.11 primer |
| Item data in files | `ItemStack` codecs | `ItemStackTemplate` (+ `ItemInstance`); `FluidStackTemplate` | 26.1 | 26.1 post and primer |
| Block entity type | `BlockEntityType.Builder` | builder removed, public constructor / AT | 1.21.2 | 1.21.2 primer |
| NBT | `CompoundTag` getters | `Optional` getters, codec `store`/`read` | 1.21.5 | 1.21.5 primer |
| Block entity and entity save | `saveAdditional(CompoundTag, Provider)` | `saveAdditional(ValueOutput)` / `loadAdditional(ValueInput)`; entities `addAdditionalSaveData(ValueOutput)` | 1.21.6 | [1.21.6 primer](https://github.com/neoforged/.github/blob/main/primers/1.21.6/index.md) |
| Attachments | `INBTSerializable`, `serialize(Codec)` | `ValueIOSerializable`, `serialize(MapCodec)`; item attachments superseded by components | 1.21.6 (**verify** exact) | [attachments docs](https://docs.neoforged.net/docs/datastorage/attachments/) |
| Item / fluid / energy transfer | `IItemHandler`, `IFluidHandler`, `IEnergyStorage`, `Capabilities.ItemHandler` | `ResourceHandler<ItemResource>`, `ResourceHandler<FluidResource>`, `EnergyHandler`, `Transaction`; `Capabilities.Item.BLOCK/ENTITY/ITEM` | 1.21.9 (NeoForge 21.9) | [Transfer Rework](https://neoforged.net/news/21.9-transfer-rework/), [capabilities docs](https://docs.neoforged.net/docs/inventories/capabilities) |
| Creative tabs | `BuildCreativeModeTabContentsEvent`, `CreativeModeTab.builder()` | same | unchanged per 26.1 docs | [items docs](https://docs.neoforged.net/docs/items/) |
| Networking payloads | `RegisterPayloadHandlersEvent`, `CustomPacketPayload`, `StreamCodec` | same shape; `RegisterClientPayloadHandlersEvent`, `ClientPacketDistributor.sendToServer`, `Identifier` ids | 21.x to 26.1 (**verify** line) | [payload docs](https://docs.neoforged.net/docs/networking/payload/) |
| Key mappings and input | GLFW key codes | `KeyMapping.Category` (**verify**); SDL via `InputConstants` / `SDLScancode`; `KeyEvent#scancode` renamed `keycode` | 1.21.9 / 26.3 | [26.3 primer](https://github.com/neoforged/.github/blob/main/primers/26.3/index.md) |
| GUI rendering | `GuiGraphics#render*`, `Screen#render` | `GuiRenderState`, then `GuiGraphicsExtractor`, `Screen#extractRenderState`; `Hud` class | 1.21.6 / 26.1 / 26.2 | 1.21.6 primer, 26.1 post, [26.2 primer](https://github.com/neoforged/.github/blob/main/primers/26.2/index.md) |
| Entity rendering | `EntityRenderer#render(entity,...)` | `EntityRenderState` + `extractRenderState`; `submit(state, PoseStack, SubmitNodeCollector, CameraRenderState)`; block-entity renderers likewise | 1.21.2 / 1.21.9 | 1.21.2 and [1.21.9](https://github.com/neoforged/.github/blob/main/primers/1.21.9/index.md) primers |
| Render API | `RenderType`, Blaze3D | `RenderSetup` + `RenderTypes` (1.21.11); `MultiBufferSource` removed, feature renderers (26.2); `com.mojang.renderpearl`, `#include`, OIT (26.3); Vulkan backend (26.2) | 1.21.11 to 26.3 | 1.21.11, 26.2, 26.3 primers |
| Recipes | `RecipeProvider` as `DataProvider`, `RecipeHolder` | `RecipeProvider` + nested `Runner` (1.21.2); `RecipeSerializer` record, `ItemStackTemplate` results, `RecipeUnlockAdvancementBuilder` (26.1); reloadable registry, `BootstrapContext` (26.3) | 1.21.2 / 26.1 / 26.3 | primers 1.21.2, 26.1, 26.3 |
| Loot | `LootItemFunctionType` etc. | types unrolled to `MapCodec`s (26.1); `Holder<LootItemCondition>`, reloadable registries, `LootTableSubProvider.Factory` (26.3) | 26.1 / 26.3 | 26.1, 26.3 primers |
| GameTest | `@GameTestHolder`, `@GameTest`, templates | `BuiltInRegistries.TEST_FUNCTION` (`Consumer<GameTestHelper>`), `TEST_INSTANCE` JSON or `RegisterGameTestsEvent`; `/test`; `TestFunction` -> `TestData` | 1.21.5 | 1.21.5 primer, [GameTest docs](https://docs.neoforged.net/docs/misc/gametest/) |
| Datagen | `GatherDataEvent`, `RecipeProvider` direct | split client/server data (**verify**), `ModelProvider`/`ItemModelGenerators` for `items/`, `RegistriesDatapackGenerator` bootstrap providers, `LootTableProvider` factories | 1.21.4 to 26.3 | primers 1.21.2, 26.3; [resources docs](https://docs.neoforged.net/docs/resources/) |
| Permissions | int levels, `hasPermission(2)` | `Permission`, `PermissionSet`, `PermissionCheck`; `Commands.LEVEL_*` are checks | 1.21.11 | 1.21.11 primer |
| Saved data | string ids, `DimensionDataStorage` | `Identifier` ids, `SavedDataStorage`, game rules and weather moved into saved data | 26.1 | 26.1 primer |
| Time | `Level#getDayTime` | `WorldClock`, `ClockManager`, `Level#getOverworldClockTime` | 26.1 | 26.1 primer |
| Random | `Level#random` public | `getRandom()` | 26.1 | 26.1 primer |
| Data pack / resource pack formats | `pack_format` 48 / 34 (1.21.1) | new numbers every release; **verify** the 26.3 values and whether `pack.mcmeta` now uses range fields | every release | Misode changelog (cited by the primers) |
| `ChunkPos` | `new ChunkPos(blockPos)` | `ChunkPos.containing(blockPos)`, `pack`/`unpack` | 26.1 | 26.1 post |
| Out of scope for the course | | data-driven villager trades (26.1), dialogs and waypoints (1.21.6), palettes (26.3) | | primers |

Porting references to cite in lessons: the primer for each version (links above), the NeoForge release
posts (21.x series, 26.1), the transfer rework post, and the docs site (still has a 1.21.1 selector).

## 4. Risks and open questions

| Risk | Detail | Plan |
|---|---|---|
| Version churn | One Minecraft release per ~3 months (26.1 Mar, 26.2 ~Jun, 26.3 Sep, 26.4 due ~Dec), each with API breaks; the 26.2 and 26.3 primers alone are hundreds of KB. | Pin 26.3 and freeze. Every lesson names the version it is true for. Re-cut policy below. |
| Re-cut cost | A new pin touches `what-moved` quizzes, the cheat sheet, every starter/solution, and library pins. | Re-cut **at most twice a year** (pick the NeoForge line the community settles on, as 1.21.1 did). Between re-cuts, a patch bump of NeoForge inside the same Minecraft version needs only a content-check run. Budget ~1-2 days per re-cut with a script that diffs the primer headings. |
| Library lag | EMI, KubeJS, owo-lib have no 26.3 NeoForge build; Curios is beta; GeckoLib jumped to 5; JEI jumps a major version each release (31 for 26.3). | World 10 lists blocked quests instead of faking them; revisit each at re-cut. |
| Architectury | API 22.0.5 exists for 26.3, but the project's relevance after 26.x is unclear (the reference mod avoids it); Loom must support unobfuscated 26.x and JDK 25 (**verify**). | World 9 stays small (5 quests); drop it from this track if the spike fails. |
| Research gaps | Primers for 1.21.4, 1.21.7, 1.21.8, 1.21.10 not read; the 26.1, 26.2, 26.3 primers and 1.21.x primers were read only to ~100 KB each; pack format numbers, `GatherDataEvent` split, attachment signature, `unitTest`/`EphemeralTestServerProvider` names, GameTest enable property all unconfirmed. | Every **verify** tag above; spike resolves them before content is written. |
| Rendering rewrite | Rendering changed in 1.21.2, 1.21.6, 1.21.9, 1.21.11, 26.2, 26.3, and is not gradable headless. | Keep rendering quests to wiring (registration, paths, state extraction) plus look-checks; avoid custom shaders and `RenderPipeline` work entirely. |
| Disk | Second dev environment: another ~2.4 GB shared cache (JDK 25 + Gradle 9.2.1 + MC 26.3) plus ~0.5 GB per active world, not shared with the 1.21.1 track. | Setup quest shows both numbers; workspace cleanup opt-in; mention `dev-clean.ps1`-style cleanup. |
| JDK 25 not installed | The machine has JDK 21 only (TRACKS.md). | `java-25` Setup quest installs Temurin 25 via winget after asking; `jdk = 25` in `track.toml`. |
| Gating | `requires` into `neoforge/...` makes this track unreachable until 1.21.1 content is done. | Open question 1. |
| Boss costs | Every world has a ported boss that re-asserts its 1.21.1 objective names; tests must be rewritten, not copied. | Author each from the 1.21.1 solution via a port diff; content-check builds starter and solution in a temp workspace. |
| Name clash | Vanilla `/waypoint` (1.21.6) clashes with the world 8 boss command root. | Rename the command (**verify** current vanilla name). |

### Open questions for the user
1. Gate this track behind the 1.21.1 one (current plan, via `requires`), or let a newcomer start here
   with the unchanged quests shipped as a "baseline" the learner skips?
2. Is quarterly drift acceptable, i.e. freeze at 26.3 and re-cut once per ~6 months, or track every
   Minecraft release?
3. Keep the Architectury world in this track (5 quests) or drop it until Architectury and Loom stabilise on 26.x?
4. Should blocked library quests (EMI, KubeJS, owo-lib) stay listed as placeholders or be omitted until they ship?

### First engine tasks implied
1. Extend the 1-hour spike (neoforge.md) to 26.3: cold and warm timings with JDK 25 / Gradle 9.2.1, and
   resolve every **verify** above on a fresh MDK 26.3 workspace.
2. `track.toml jdk = 25` and per-track `JAVA_HOME` in the gradle runner.
3. content-check: collision check between `neoforge` and `neoforge-latest` ids; "ported baseline" copy
   step in quest 01 of each world.
