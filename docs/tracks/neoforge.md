# Track plan: Minecraft, NeoForge 1.21.1 + Architectury + ecosystem libs

Planning doc for `content/tracks/neoforge` (runner `gradle`, `requires java/02-oop`). Fabric is planned
separately (`docs/tracks/fabric.md`). Versions checked 2026-10-19 against Maven metadata; items marked
**verify** are from memory or third-party pages and need a spike before content is written.
Timings are **estimates, not measurements**; the first engine task is a 1-hour spike that measures them.

Reference project studied (read-only): `X:\Dev\Minecraft\effortless-building-multi`. Facts worth copying:
- Three modules `common`, `neoforge`, `fabric`. **No Architectury**: `common` uses ModDevGradle with
  `neo_form_version` (vanilla only), `neoforge` uses ModDevGradle `net.neoforged.moddev`, `fabric` uses
  Fabric Loom, and `common/.../platform/Services.java` bridges them with `ServiceLoader`. Forgix merges
  both jars. World 9 teaches **both** this and the Architectury way, so learners can read real mods.
- `gradle.properties` pins `minecraft_version`, `neo_form_version`, Parchment `1.21` / `2024.11.10`,
  `neoforge_version=21.1.80`, `neoforge_loader_version_range=[4,)`, `org.gradle.jvmargs=-Xmx3G`.
  It sets `org.gradle.daemon=false`; Oxide must **not** copy that (we want the warm daemon).
- Client UI tests exist as a custom `runUiTest` run type with system properties. Same idea as our
  "client look-check" quests, but not gradable headless.

## 1. Toolchain and runner

### Versions to pin

| Thing | Pin | Notes |
|---|---|---|
| JDK | 21 (Adoptium 21.0.x, detected by Oxide) | Minecraft 1.21.x requires 21 |
| Minecraft | 1.21.1 | NeoForge line `21.1.x` |
| NeoForge | **21.1.264** (newest 21.1.x on 2026-10-19; freeze at content-writing time) | Effortless Building pins 21.1.80; any 21.1.x is API-compatible. Other lines exist: 21.3 (1.21.3), 21.4, 21.5, 21.8, 21.10, 21.11 and a new `26.x` major (26.1 to 26.3) under Mojang's year-based versioning. 1.21.1 still gets fixes. |
| Build plugin | **ModDevGradle `net.neoforged.moddev` 2.0.148** | Latest 2.0.x. NeoGradle rejected, see below |
| Mappings | Mojang official + **Parchment `1.21` / `2024.11.17`** (parameter names + javadoc) | Same combo as the reference mod (it uses 2024.11.10) |
| Gradle | wrapper bundled in the MDK (8.x) | MDG 2.x needs Gradle 8.8 or newer; **verify** the exact wrapper when taking the MDK |
| JUnit | 5 (Jupiter) via MDG `unitTest` + `net.neoforged:testframework` | **verify** coordinates |
| Architectury API | `13.0.x` for 1.21.1 (13.0.11 seen on a mirror, **verify** on the official maven) | needs Architectury Loom; pin the plugin version from the Architectury template at writing time |

**ModDevGradle vs NeoGradle:** MDG is the NeoForge-recommended default for new mods (single plugin,
fewer moving parts, faster setup, built-in `unitTest` and `gameTestServer` run types, works without
an IDE). NeoGradle is for projects that need its extra patching hooks. We use MDG everywhere,
including the `neoforge` module of world 9.

### One world = one workspace

```
<oxide data>/workspaces/neoforge/<world>/         # e.g. 01-registries
  gradlew(.bat), gradle/wrapper/                  # from the MDK, never touched by quests
  build.gradle, settings.gradle, gradle.properties  # engine-owned (see "replace")
  src/main/java/com/oxide/learn/...               # learner code (mod classes)
  src/main/resources/META-INF/neoforge.mods.toml
  src/test/java/oxide/...                         # quest JUnit tests, one class per quest
  src/main/java/com/oxide/learn/gametest/...      # GameTests live in main so the gametest server loads them
  src/main/templates/META-INF/neoforge.mods.toml  # MDK template, filled from gradle.properties
```

- Quest 1 of the world copies the MDK base + its `starter/`. Later quests **merge** their `starter/`
  files, never overwriting an existing learner file (TRACKS.md rule), except those in `replace = [...]`.
- **Contract classes.** A quest test may only touch classes/signatures shipped by an earlier quest's
  starter (for example `ModItems.RUBY`, `LedgerData`), never what the learner invented. If a learner
  solved quest N differently, quest N+1 still compiles because its starter also ships the stub
  signatures with `// TODO`. Provide a "reset to reference solution of previous quest" action for
  anyone who got stuck (merge from `solution/`).
- `build.gradle` is engine-owned and identical for every world of a family (same pins, same
  `unitTest`/`runs` blocks); worlds differ only in `gradle.properties` (`mod_id`) and dependencies
  (world 10 adds JEI/EMI/Curios/GeckoLib/Jade/KubeJS/owo `compileOnly` + one `runtimeOnly` at a time).
- Worlds 0-8 and 10-11: `neoforge` single-module project. World 9 and 11: Architectury three-module
  project (roughly twice the build output). Workspace per world is deleted when the world is
  finished and the learner opts in (`Free 600 MB`); progress stays.
- New engine feature requested for datagen quests: `pre = ["runData"]` in quest.toml, a Gradle task
  run before `test` (tests then read `src/generated/resources`). Same for world 9 `assemble`.

### JUnit without launching Minecraft

Three tiers, cheapest first. Quest authors choose the tier; lessons say which.

| Tier | What it can test | How | Cost |
|---|---|---|---|
| **A. Plain Java** | Pure logic: codecs (`DataFixer`-free `Codec`/`StreamCodec` over primitives), Brigadier dispatchers, config specs, shape/maths, JSON files, `neoforge.mods.toml` | Put logic in classes that do not touch registries; `ModConfigSpec` and `CommandDispatcher<T>` work without Minecraft | ~1-3 s warm |
| **B. Bootstrapped** | Anything touching vanilla registries/blocks/items/components: `ItemStack`, `BlockState`, NBT round-trip | MDG `unitTest { enable(); testedMod = mods.<name> }`; the test launcher bootstraps registries and loads the mod (or `Bootstrap.bootStrap()` manually). No window, no world | ~5-10 s warm (JVM + registry bootstrap) |
| **C. Ephemeral server** | Anything needing a `ServerLevel`: block entity ticks, capabilities, events, commands with a real player | `@ExtendWith(EphemeralTestServerProvider.class)` from the NeoForge test framework (a headless server in the test JVM) | ~10-20 s warm |

**verify** names and annotations in the spike. Rule: if a quest can be tier A, it is tier A; the
lesson teaches "split logic from registration" as a design habit (it is also what makes mods testable).

What **needs** a Minecraft bootstrap: creating items/blocks/entities, any `Registry` lookup,
`ItemStack`, `CompoundTag` via components, `Component.translatable` resolution (needs language).
Avoid by: pass primitives and `Identifier`/`ResourceLocation` strings, keep calculation in
static methods, inject suppliers instead of reading `ModItems.X` inside logic.

### GameTests for bosses

- NeoForge 1.21.1 GameTest framework: tests registered with the 1.21.1 API (`@GameTestHolder` +
  `@GameTest` with `@EmptyTemplate`, or `RegisterGameTestsEvent`; **verify**, this changes in 1.21.5+).
  MDG run type `gameTestServer` with `-Dneoforge.enabledGameTestNamespaces=<mod_id>` runs them
  headless and exits nonzero when a required test fails.
- Oxide runs `gradlew test runGameTestServer` for bosses. To get per-test objectives without scraping
  log text, the workspace ships a 40-line `OxideGameTestReporter` that calls
  `GlobalTestReporter.replaceWith(new JUnitLikeTestReporter(file))` (vanilla class, writes JUnit XML;
  Fabric uses the same trick; **verify** it is present in 1.21.1). Output: `build/oxide/gametest.xml`,
  parsed by the same JUnit-XML parser as `build/test-results/test/*.xml`. Fallback: parse the log
  lines `... passed/failed` from the server output.
- Test names: GameTest method name == objective `test`. Quest.toml lists them like JUnit names.
- Boss staging (CONTENT.md: first objective passes early): tier A/B JUnit objectives first, GameTest
  objectives last, so the learner sees ticks before the slow run.
- Flakiness: always use `@GameTest(timeoutTicks=...)`, `helper.succeedWhen`, no real-time waits, and a
  fixed empty template; reseed randomness (`helper.getLevel().random.setSeed`).

### Time and size (estimates, to be measured)

| Moment | Estimate |
|---|---|
| First ever run in a fresh `GRADLE_USER_HOME` | **3-8 min**, ~1.5-2.5 GB downloaded/cached (Gradle dist ~130 MB, Minecraft client/server jars, libraries, assets ~0.4 GB, NeoForm decompile + recompile via the NeoForm runtime, then Parchment) |
| Second workspace with the same pins | 20-40 s (cache shared via `GRADLE_USER_HOME`; each workspace adds ~0.3-0.6 GB `build/` and `.gradle/`) |
| `gradlew test`, tier A, warm daemon | 2-5 s (TRACKS.md number holds) |
| `gradlew test`, tier B/C, warm daemon | 6-20 s |
| `gradlew runGameTestServer`, warm | 25-60 s (JVM start, world load, tests) |
| `gradlew runData` | 20-40 s |
| `gradlew runClient` | 30-90 s to title screen (never run by the grader) |

Correction to TRACKS.md: the "~2 GB per dev environment" is mostly the **shared** Gradle cache, paid
once per Minecraft/NeoForge pin. Per world workspace is ~0.5 GB. Disk guidance: show "uses 2.4 GB
shared + 0.5 GB per active world" on the Setup quest; one NeoForge pin only (do not mix in Fabric's
1.21.1 Loom cache needlessly; Fabric shares the vanilla jars only if both use the same Minecraft
download directory, which they do not, expect ~1.5 GB more for Fabric).

### Not gradable, and what the lesson does

Rendering (screens, models, entity renderers, particles, GeckoLib animation) and key input cannot be
asserted headless. Pattern, always the same three beats:
1. **Graded code**: the exercise checks the *wiring* (renderer registered, texture path exists,
   layer definition registered, keymapping registered), which is what usually breaks.
2. **Run the client, look**: lesson ends with `gradlew runClient` instructions and a reference
   screenshot.
3. **Look-check read quest** (15 XP, 3 min) whose quiz asks something only visible in game ("what
   colour is the progress bar at 100%?"). Cheap, honest, and keeps the streak going.

## 1b. Where quests fit the Oxide format

- Exercise 3-15 min, 1-4 objectives; boss 40-90 min with 4-6 objectives (staged, first early).
- Starter must **compile** as shipped (`todo()`-style: `throw new UnsupportedOperationException("TODO")`),
  so a bad quest shows red ticks not a compiler wall. `starter_fails_to_compile` only where the lesson
  is about a compile error.
- Setup world's first quest is a read quest (< 5 min), as required.


## 2. Worlds

Quest id = `neoforge/<NN-world>/<NN-slug>`. Kind: `ex` exercise, `read` quiz quest, `boss`. **Client** = the lesson sends the learner to `runClient` (rendering, input); the graded objectives still run headless. Quick wins are the 30-XP / 4-6 min rows.

| World | Title | Quests | XP | Minutes |
|---|---|---|---|---|
| 00 | Setup | 8 | 495 | 81 |
| 01 | Registries | 10 | 770 | 140 |
| 02 | Data and datagen | 10 | 725 | 127 |
| 03 | Events and attachments | 10 | 775 | 141 |
| 04 | Interaction and menus | 10 | 825 | 151 |
| 05 | Networking | 10 | 800 | 137 |
| 06 | Entities | 10 | 845 | 153 |
| 07 | World | 9 | 750 | 139 |
| 08 | Commands and config | 10 | 845 | 153 |
| 09 | Architectury (multi-loader) | 11 | 955 | 186 |
| 10 | Ecosystem libraries | 11 | 980 | 184 |
| 11 | Capstone | 7 | 785 | 149 |
| | **Total** | **116** | **9550** | **1741** (~29.0 h) |

### World 00: Setup (`00-setup`, 8 quests, 495 XP)

Toolchain check, the MDK, first run. Under 5 minutes to the first tick.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `jdk-and-gradle` | What gets downloaded, where it lives, why the first run is slow. | quiz | 4 | 15 | read |  |
| 02 | `name-your-mod` | Set `mod_id`, name and group in `gradle.properties`. | `mod_id_is_valid_slug`<br>`mod_name_not_default` | 5 | 30 | ex |  |
| 03 | `mods-toml` | Fill `neoforge.mods.toml`: loader range, NeoForge dependency range. | `toml_has_required_keys`<br>`dependency_range_covers_1_21_1` | 8 | 40 | ex |  |
| 04 | `hello-mod` | Mod entry class with a constructor that takes `IEventBus` and logs. | `mod_class_has_mod_id_constant`<br>`mod_id_matches_properties` | 8 | 40 | ex |  |
| 05 | `run-the-client` | `runClient`, open the Mods screen, find your mod. Look-check quiz. | quiz | 4 | 15 | read | yes |
| 06 | `setup-listener` | Register a `FMLCommonSetupEvent` listener whose logic is a plain method. | `setup_marks_ready`<br>`listener_registered_on_mod_bus` | 8 | 40 | ex |  |
| 07 | `parchment-names` | Mojang names + Parchment parameter names; reading decompiled vanilla. | quiz | 4 | 15 | read |  |
| 08 | `boss-welcome-mod` | Players get a greeting on join and a visit counter that survives restarts. | `greeting_formats_player_name`<br>`visit_counter_increments`<br>`greeting_sent_on_join`<br>`mod_loads_in_gametest_server` | 40 | 300 | **boss** |  |

### World 01: Registries (`01-registries`, 10 quests, 770 XP)

Items, blocks, block entities, data components, creative tab.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `deferred-register` | Why `DeferredRegister` exists and when registration actually runs. | quiz | 4 | 15 | read |  |
| 02 | `first-item` | Register an item with id, stack size and rarity. | `item_registered_with_id`<br>`item_stack_size_is_16` | 8 | 40 | ex |  |
| 03 | `food-item` | `FoodProperties` and a use animation. | `food_nutrition_set`<br>`food_has_eat_effect` | 8 | 45 | ex |  |
| 04 | `creative-tab` | Add the item to your own creative tab. | `tab_registered`<br>`item_in_tab_contents` | 5 | 30 | ex |  |
| 05 | `first-block` | Block + `BlockItem`, hardness, sound type. | `block_registered`<br>`block_item_registered`<br>`hardness_and_sound_set` | 9 | 45 | ex |  |
| 06 | `blockstate-property` | A `lit` boolean property with default state. | `default_state_unlit`<br>`lit_property_toggles` | 10 | 55 | ex |  |
| 07 | `block-entity` | Block entity type, NBT save/load round-trip. | `be_type_registered`<br>`be_saves_and_loads_nbt`<br>`be_valid_blocks_set` | 12 | 65 | ex |  |
| 08 | `ticking-be` | Server-only ticker that counts burn time. | `ticker_counts_up`<br>`ticker_skipped_on_client` | 12 | 65 | ex |  |
| 09 | `data-components` | Custom `DataComponentType` with a codec; store charge on a stack. | `component_registered`<br>`component_codec_roundtrips`<br>`stack_stores_component` | 12 | 60 | ex |  |
| 10 | `boss-pocket-lantern` | A lantern block that glows and burns down, and a rechargeable item form. | `lantern_block_registered`<br>`lantern_light_level_toggles`<br>`charge_component_roundtrips`<br>`burn_time_persists`<br>`placed_lantern_emits_light` | 60 | 350 | **boss** |  |

### World 02: Data and datagen (`02-data`, 10 quests, 725 XP)

Never hand-write JSON again. Providers generate models, lang, recipes, loot, tags.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `datagen-wiring` | `GatherDataEvent` handler adds providers; `runData` writes `src/generated`. | `providers_registered`<br>`generated_dir_configured` | 8 | 40 | ex |  |
| 02 | `lang-provider` | English names for item, block and tab. | `lang_has_item_name`<br>`lang_has_block_name`<br>`lang_has_tab_name` | 6 | 35 | ex |  |
| 03 | `item-models` | Generated item model with the right parent. | `item_model_json_exists`<br>`item_model_parent_correct` | 8 | 45 | ex |  |
| 04 | `block-models` | Blockstate + block model + item model for a block. | `blockstate_has_variants`<br>`block_model_texture_set` | 10 | 55 | ex |  |
| 05 | `tags` | Block and item tags, mineable and tool-tier. | `block_in_mineable_tag`<br>`tool_tier_tag_set` | 4 | 30 | ex |  |
| 06 | `recipes` | Shaped, shapeless, smelting, with unlock criteria. | `shaped_recipe_json`<br>`shapeless_recipe_json`<br>`recipe_has_unlock_criterion` | 10 | 55 | ex |  |
| 07 | `loot-tables` | Block drops itself; silk-touch-only variant. | `block_drops_itself`<br>`silk_touch_only_drop` | 10 | 55 | ex |  |
| 08 | `data-maps` | Furnace fuel and compost values via data maps. | `fuel_datamap_json`<br>`compost_datamap_json` | 8 | 45 | ex |  |
| 09 | `handwritten-vs-generated` | When to still hand-write JSON (data pack overrides, optional files). | quiz | 3 | 15 | read |  |
| 10 | `boss-ruby-pack` | A full content set from one datagen run: ore block, gem, tools recipe set, loot, tags, lang. | `datagen_is_deterministic`<br>`all_registered_objects_have_models`<br>`all_blocks_have_loot_tables`<br>`recipes_unlock_correctly`<br>`lang_has_no_missing_keys` | 60 | 350 | **boss** | yes |

### World 03: Events and attachments (`03-events`, 10 quests, 775 XP)

Game bus vs mod bus, data attachments, item/block capabilities.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `event-bus-tour` | Mod bus, game bus, `@EventBusSubscriber`, sides. | quiz | 4 | 15 | read |  |
| 02 | `first-subscriber` | Handle `PlayerEvent.PlayerLoggedInEvent` with logic in a pure method. | `subscriber_registered`<br>`handler_logic_pure` | 8 | 40 | ex |  |
| 03 | `cancel-and-priority` | `EventPriority` and cancelling a `ICancellableEvent`. | `high_priority_runs_first`<br>`cancelled_event_stops_default` | 5 | 30 | ex |  |
| 04 | `incoming-damage` | Halve fall damage when wearing your boots (`LivingIncomingDamageEvent`). | `fall_damage_halved_with_boots`<br>`other_damage_untouched` | 10 | 50 | ex |  |
| 05 | `block-break-event` | Veto breaking a protected block, tell the player why. | `protected_block_cancels_break`<br>`message_sent_to_player` | 10 | 50 | ex |  |
| 06 | `attachments` | Register a player `AttachmentType` with codec and default. | `attachment_registered`<br>`attachment_codec_roundtrip`<br>`attachment_has_default` | 12 | 65 | ex |  |
| 07 | `copy-on-death` | Keep the attachment across death with `copyOnDeath`. | `attachment_survives_clone`<br>`other_attachment_not_copied` | 8 | 45 | ex |  |
| 08 | `item-capability` | Expose an `IItemHandler` on an item stack. | `item_handler_capability_registered`<br>`insert_and_extract_work` | 12 | 65 | ex |  |
| 09 | `block-capability` | Expose `IEnergyStorage` on a block entity, query it from a neighbour. | `energy_capability_registered`<br>`neighbour_can_extract` | 12 | 65 | ex |  |
| 10 | `boss-miners-ledger` | A ledger item reads a per-player blocks-mined counter that survives death and relog. | `counter_increments_on_break`<br>`counter_survives_death`<br>`counter_survives_relog`<br>`ledger_reports_count`<br>`capability_exposes_ledger_slot` | 60 | 350 | **boss** |  |

### World 04: Interaction and menus (`04-interaction`, 10 quests, 825 XP)

Right-click, hand rules, containers, screens.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `use-item` | `Item.use` on server: cooldown and result type. | `use_returns_success_server_side`<br>`cooldown_applied` | 8 | 45 | ex |  |
| 02 | `use-on-block` | `useOn` and `UseOnContext`: place a torch-like block on a face. | `places_on_clicked_face`<br>`fails_on_water` | 10 | 50 | ex |  |
| 03 | `interact-entity` | `interactLivingEntity`: feed or tag a mob. | `feeds_target_mob`<br>`stack_shrinks_in_survival` | 8 | 45 | ex |  |
| 04 | `sound-and-particles` | Play a sound and spawn particles on the right side. | `sound_played_on_server`<br>`particles_sent_to_tracking` | 4 | 30 | ex |  |
| 05 | `block-use` | `useWithoutItem` / `useItemOn` toggling a lever-like block. | `empty_hand_toggles`<br>`item_hand_passes_through` | 10 | 50 | ex |  |
| 06 | `menu-and-slots` | `MenuType`, slot layout, `quickMoveStack`. | `menu_type_registered`<br>`slot_layout_matches`<br>`quick_move_moves_stack` | 14 | 70 | ex |  |
| 07 | `screen-class` | `AbstractContainerScreen`, background, labels, registered via `RegisterMenuScreensEvent`. | `screen_registered_for_menu`<br>`gui_texture_path_exists` | 12 | 55 | ex | yes |
| 08 | `look-check-screen` | Open the chest in game and compare with the reference screenshot. | quiz | 3 | 15 | read | yes |
| 09 | `container-data` | `ContainerData` syncing a progress bar value to the client. | `data_slot_count_matches`<br>`progress_reaches_client` | 12 | 65 | ex |  |
| 10 | `boss-sorting-chest` | A chest block with a menu that merges and sorts stacks, contents persist. | `menu_opens_on_right_click`<br>`quick_move_respects_slots`<br>`sort_merges_stacks`<br>`contents_persist_in_world`<br>`hopper_can_insert` | 70 | 400 | **boss** |  |

### World 05: Networking (`05-networking`, 10 quests, 800 XP)

Payloads, StreamCodecs, handlers, never trusting the client.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `payload-model` | Client to server, server to client, `CustomPacketPayload`. | quiz | 4 | 15 | read |  |
| 02 | `stream-codecs` | Build a `StreamCodec` for a record with a `BlockPos` list. | `stream_codec_roundtrips`<br>`empty_list_roundtrips` | 5 | 30 | ex |  |
| 03 | `first-payload` | A payload record, its `Type`, its codec. | `payload_codec_roundtrip`<br>`payload_type_id_matches_mod` | 10 | 55 | ex |  |
| 04 | `register-payloads` | `RegisterPayloadHandlersEvent`, version string, direction. | `registrar_version_set`<br>`both_directions_registered` | 10 | 55 | ex |  |
| 05 | `client-to-server` | Server handler that validates distance and permission. | `rejects_far_players`<br>`accepts_valid_request`<br>`handler_runs_on_main_thread` | 12 | 60 | ex |  |
| 06 | `server-to-client` | Send to tracking players when state changes. | `sent_to_tracking_players_only`<br>`client_handler_updates_cache` | 12 | 60 | ex |  |
| 07 | `sync-attachment` | Make a world-4 style attachment auto-sync. | `attachment_syncs_to_owner`<br>`sync_codec_roundtrips` | 10 | 55 | ex |  |
| 08 | `keybind-trigger` | `KeyMapping` that sends a payload on press. | `keymapping_registered`<br>`press_sends_payload` | 10 | 55 | ex | yes |
| 09 | `trust-nothing` | Why client data is hostile, `Dist`, and the class-loading trap. | quiz | 4 | 15 | read |  |
| 10 | `boss-ping-wand` | Press a key, the server validates, everyone nearby sees a marker particle. | `payload_roundtrips`<br>`server_rejects_spam`<br>`marker_reaches_nearby_players`<br>`no_client_class_on_server`<br>`cooldown_enforced_server_side` | 60 | 400 | **boss** | yes |

### World 06: Entities (`06-entities`, 10 quests, 845 XP)

A custom mob: attributes, goals, renderer, loot.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `entity-types` | `EntityType`, builder, size, tracking range. | quiz | 4 | 15 | read |  |
| 02 | `register-entity` | Register the type and its attributes. | `entity_type_registered`<br>`attributes_registered`<br>`entity_spawns_in_world` | 10 | 55 | ex |  |
| 03 | `attribute-tuning` | Health, speed, follow range. | `attribute_values_in_range`<br>`mob_category_correct` | 8 | 45 | ex |  |
| 04 | `spawn-egg` | Spawn egg item and spawn placement. | `spawn_egg_registered`<br>`spawn_placement_rules_set` | 5 | 30 | ex |  |
| 05 | `goal-basics` | Goal selector and priorities: flee when hurt. | `flee_goal_registered`<br>`flee_goal_activates_when_hurt` | 12 | 60 | ex |  |
| 06 | `custom-goal` | Write a goal that walks to a dropped item. | `goal_can_use_conditions`<br>`goal_moves_to_item` | 14 | 70 | ex |  |
| 07 | `renderer` | Model layer, renderer registration (humanoid-style reuse is fine). | `renderer_registered`<br>`layer_definition_registered` | 12 | 55 | ex | yes |
| 08 | `look-check-renderer` | Spawn it with the egg and compare with the reference image. | quiz | 3 | 15 | read | yes |
| 09 | `entity-loot` | Loot table with looting bonus. | `loot_table_json_valid`<br>`looting_increases_count` | 10 | 50 | ex |  |
| 10 | `boss-magpie` | A mob that picks up dropped items and delivers them to its nest block. | `magpie_spawns`<br>`picks_up_item`<br>`delivers_to_nest`<br>`ignores_when_hurt`<br>`persists_across_reload` | 75 | 450 | **boss** |  |

### World 07: World (`07-world`, 9 quests, 750 XP)

Ores, structures, a dimension taste.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `worldgen-json` | Configured feature, placed feature, biome modifier: the chain. | quiz | 4 | 15 | read |  |
| 02 | `ore-feature` | Configured and placed feature for an ore (datagen bootstrap). | `ore_feature_json_valid`<br>`placement_modifiers_ordered` | 10 | 55 | ex |  |
| 03 | `biome-modifier` | Add the ore to the overworld only. | `biome_modifier_adds_ore`<br>`targets_overworld_tag` | 12 | 60 | ex |  |
| 04 | `vein-tuning` | Vein size and count sanity ranges. | `vein_size_in_range`<br>`count_in_range` | 5 | 30 | ex |  |
| 05 | `structure-template` | Save a small NBT structure with `/structure`, load it in a test. | `template_file_exists`<br>`template_loads_and_has_blocks` | 10 | 55 | ex |  |
| 06 | `structure-set` | Structure, structure set, spacing and biome tag. | `structure_json_valid`<br>`spacing_greater_than_separation` | 12 | 60 | ex |  |
| 07 | `dimension-intro` | Dimension type vs dimension vs level stem. | quiz | 4 | 15 | read |  |
| 08 | `dimension-type` | A tiny custom dimension type and level stem. | `dimension_type_json_valid`<br>`level_stem_references_type` | 12 | 60 | ex |  |
| 09 | `boss-sunken-ruin` | Ore plus a ruin structure that places loot chests; locate it with `/locate`. | `ore_generates_in_test_chunk`<br>`template_places_loot_chest`<br>`structure_set_valid`<br>`biome_modifier_valid`<br>`loot_chest_has_contents` | 70 | 400 | **boss** |  |

### World 08: Commands and config (`08-commands-config`, 10 quests, 845 XP)

Brigadier, ModConfigSpec, config screens.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `brigadier-literals` | Pure Brigadier: a `CommandDispatcher` tested without Minecraft. | `literal_command_runs`<br>`unknown_literal_fails` | 8 | 45 | ex |  |
| 02 | `brigadier-arguments` | Integer and string arguments, parse errors. | `int_argument_parsed`<br>`bad_argument_fails` | 10 | 50 | ex |  |
| 03 | `command-feedback` | `sendSuccess` with a translatable component. | `feedback_uses_translation_key` | 5 | 30 | ex |  |
| 04 | `permissions-suggestions` | `requires` permission level and a suggestion provider. | `requires_permission_level`<br>`suggestions_listed` | 10 | 55 | ex |  |
| 05 | `register-commands` | `RegisterCommandsEvent` hooks it into the game. | `command_registered_on_event`<br>`command_runs_on_server` | 10 | 55 | ex |  |
| 06 | `config-spec` | `ModConfigSpec` with ranges and defaults. | `config_defaults_valid`<br>`range_validation_rejects` | 10 | 55 | ex |  |
| 07 | `config-reload` | Cache values and refresh on `ModConfigEvent.Reloading`. | `reload_updates_cached_value`<br>`cache_not_read_before_load` | 10 | 50 | ex |  |
| 08 | `builtin-config-screen` | `IConfigScreenFactory` using NeoForge's built-in screen. | `config_screen_factory_registered` | 8 | 45 | ex | yes |
| 09 | `cloth-config-screen` | Cloth Config builder saves into the same spec. | `cloth_builder_saves_values`<br>`cloth_is_optional_dependency` | 12 | 60 | ex | yes |
| 10 | `boss-waypoints` | `/waypoint set|tp|list` per player, limit from config, saved as attachment. | `set_stores_waypoint`<br>`tp_teleports_player`<br>`limit_from_config_enforced`<br>`list_output_sorted`<br>`waypoints_persist` | 70 | 400 | **boss** |  |

### World 09: Architectury (multi-loader) (`09-architectury`, 11 quests, 955 XP)

One codebase, NeoForge and Fabric jars. Workspace is bigger (two dev environments).

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `multiloader-map` | common / neoforge / fabric, the three ways: Architectury, ServiceLoader (what Effortless Building does), MultiLoader template. | quiz | 5 | 15 | read |  |
| 02 | `loom-project` | Set up the three-module Architectury project, shared `mod_id`. | `project_has_three_modules`<br>`mod_id_shared_across_modules` | 8 | 45 | ex |  |
| 03 | `common-code` | Move pure logic to `common`, unit-test it there. | `logic_lives_in_common`<br>`common_has_no_loader_imports` | 10 | 50 | ex |  |
| 04 | `expect-platform` | `@ExpectPlatform` with the `<Class>Impl` in the loader package. | `annotation_present`<br>`neoforge_impl_in_right_package`<br>`platform_name_resolves` | 12 | 65 | ex |  |
| 05 | `platform-detect` | Architectury `Platform`: dev env, mod loaded, config dir. | `is_dev_environment_flag`<br>`mod_loaded_check` | 5 | 30 | ex |  |
| 06 | `arch-registries` | Architectury `DeferredRegister` creating an item for both loaders. | `item_registered_via_architectury`<br>`registrar_uses_mod_id` | 12 | 70 | ex |  |
| 07 | `arch-events` | Architectury events: player join and block break, one handler. | `join_handler_registered`<br>`break_handler_cancels` | 10 | 60 | ex |  |
| 08 | `arch-networking` | `NetworkManager` payload from common code. | `payload_registered_common`<br>`handler_runs_on_both_loaders` | 14 | 70 | ex |  |
| 09 | `service-loader` | The alternative: a `Services` interface loaded with `ServiceLoader`. | `service_resolved_at_runtime`<br>`missing_impl_fails_clearly` | 12 | 60 | ex |  |
| 10 | `build-both-jars` | Assemble and inspect both jars. | `neoforge_jar_has_mods_toml`<br>`fabric_jar_has_fabric_mod_json` | 8 | 40 | ex |  |
| 11 | `boss-ledger-everywhere` | Port the Miner's Ledger to common code and ship it on both loaders. | `common_logic_tests_pass`<br>`neoforge_gametest_passes`<br>`registry_via_architectury`<br>`packet_in_common`<br>`both_jars_have_metadata` | 90 | 450 | **boss** | yes |

### World 10: Ecosystem libraries (`10-ecosystem`, 11 quests, 980 XP)

JEI, EMI, Curios, GeckoLib, Jade, KubeJS, owo-lib. Each is a soft dependency.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `dependency-map` | `compileOnly` vs `runtimeOnly`, optional dependencies in `neoforge.mods.toml`. | quiz | 5 | 15 | read |  |
| 02 | `soft-dependencies` | Never load an absent mod's classes. | `no_class_load_when_absent`<br>`guard_uses_mod_list` | 5 | 30 | ex |  |
| 03 | `jei-plugin` | `IModPlugin`: a recipe category for your machine. | `plugin_uid_matches_mod`<br>`recipe_category_registered` | 12 | 60 | ex |  |
| 04 | `emi-plugin` | `EmiPlugin`: same recipes, other viewer. | `emi_plugin_registers_recipes`<br>`emi_ids_unique` | 12 | 60 | ex |  |
| 05 | `curios-slot` | Add a charm slot and make an item equip into it. | `slot_json_exists`<br>`item_has_curio_capability` | 12 | 65 | ex |  |
| 06 | `geckolib-model` | `GeoModel` with model, texture and animation paths. | `geo_paths_exist`<br>`animatable_registers_controller` | 12 | 65 | ex | yes |
| 07 | `geckolib-animation` | Animation controller and trigger; look-check at the end. | `controller_plays_idle`<br>`trigger_animation_fires` | 12 | 60 | ex | yes |
| 08 | `jade-tooltip` | Jade `IWailaPlugin` shows your block entity's state. | `jade_plugin_registers_provider`<br>`tooltip_text_contains_state` | 10 | 55 | ex |  |
| 09 | `kubejs-hook` | Expose a KubeJS binding so scripts can call your mod. | `kubejs_plugin_registers_binding`<br>`binding_returns_expected` | 12 | 60 | ex |  |
| 10 | `owo-screen` | owo-lib layout for a small screen. | `owo_layout_builds`<br>`button_has_handler` | 12 | 60 | ex | yes |
| 11 | `boss-compat-pack` | Your machine shows up in JEI, EMI and Jade, animates with GeckoLib, wears in Curios. | `jei_recipes_registered`<br>`emi_recipes_registered`<br>`jade_provider_registered`<br>`curio_slot_registered`<br>`runs_without_optional_mods` | 80 | 450 | **boss** | yes |

### World 11: Capstone (`11-capstone`, 7 quests, 785 XP)

A real multi-loader mod, built, tested and published to Modrinth.

| # | Slug | Idea | Objectives (test names) | Min | XP | Kind | Client |
|---|---|---|---|---|---|---|---|
| 01 | `plan` | Pick a feature set, write the one-page spec. | quiz | 4 | 15 | read |  |
| 02 | `mod-skeleton` | Architectury project, ids, metadata, license. | `three_modules_present`<br>`metadata_has_license_and_links` | 10 | 50 | ex |  |
| 03 | `feature-common` | Block, item, block entity and logic in common with tests. | `blocks_and_items_registered`<br>`block_entity_roundtrips`<br>`logic_unit_tests_pass` | 15 | 70 | ex |  |
| 04 | `feature-loaders` | Wire events, networking and config on both loaders. | `event_handlers_registered`<br>`payload_registered`<br>`config_spec_valid` | 15 | 70 | ex |  |
| 05 | `metadata-polish` | Icon, description, changelog. | `icon_file_exists`<br>`changelog_not_empty` | 5 | 30 | ex |  |
| 06 | `modrinth-config` | Minotaur Gradle plugin configured; dry run produces the upload plan. | `minotaur_configured`<br>`version_type_set` | 10 | 50 | ex |  |
| 07 | `boss-publish` | Ship it: both jars, GameTests green, Modrinth page live (the manual final step). | `both_jars_build`<br>`gametest_suite_passes`<br>`feature_logic_covered`<br>`no_client_code_in_common`<br>`dependencies_declared`<br>`changelog_matches_version` | 90 | 500 | **boss** | yes |

## 3. Libraries

Versions for NeoForge 1.21.1, checked 2026-10-19 (Maven/Modrinth metadata unless noted).

| Library | Version | Taught in | What the quest teaches | The one thing that bites |
|---|---|---|---|---|
| Architectury API | 13.0.x (13.0.11 seen on a mirror, **verify**) | 09 | `@ExpectPlatform`, shared `DeferredRegister`, events, `NetworkManager` | `@ExpectPlatform` needs the exact `<package>.<loader>.<Class>Impl` name, otherwise it fails at **runtime** with a missing class, not at compile. Also: Architectury API is a runtime mod dependency your players must install |
| Architectury Loom | pin from the template at writing time | 09 | three-module project, `loom.platform` | Loom and ModDevGradle do not mix in one module; the reference mod avoids Loom on NeoForge for this reason |
| Cloth Config | 15.0.140 (NeoForge, 1.21.1) | 08 | config screen builder bound to a `ModConfigSpec` | Cloth saves its own values; you must write them back into the spec and call `save()`, or the file and the screen disagree |
| JEI | 19.69.0.456 (`jei-1.21.1-neoforge-api`) | 10 | `IModPlugin`, recipe category, `@JeiPlugin`-style registration | Compile against the **api** artifact, run against the full one; touching JEI classes without a guard crashes servers without JEI |
| EMI | 1.1.x line for 1.21.1 (**verify** exact; maven host 403s the fetcher) | 10 | `EmiPlugin`, `EmiRecipe` | EMI reads JEI-style data poorly; ship native EMI recipes, and do not register the same ids in both viewers |
| Curios API | 9.5.1+1.21.1 | 10 | slot JSON, `ICurio` capability, rendering | Slots are data: forgetting the slot JSON **and** the tag makes the item silently un-equippable. Accessories (1.1.0-beta.54+1.21.1) is the competing API; track teaches Curios, mentions Accessories |
| GeckoLib 4 | 4.9.3 | 10 | `GeoModel`, `GeoEntity`/`GeoBlockEntity`, controllers, triggers | Resource paths are strict (`geo/*.geo.json`, `animations/*.animation.json`); a typo gives an invisible model with only a log warning |
| Jade | 15.10.6 (NeoForge) | 10 | `IWailaPlugin`, `IBlockComponentProvider`, server data provider | Data shown on the client must come through the **server** data provider; reading the block entity on the client shows stale or empty values |
| KubeJS | 2101.7.2-build.377 | 10 | `KubeJSPlugin` binding, event groups | Needs Rhino/Architectury as dependencies; scripts see your classes only if you allowlist them. Version moves fast, pin and freeze |
| owo-lib | 0.12.15.6-beta.1+1.21 | 10 | UI layout tree, `OwoUIAdapter` screen | beta versions only; layouts are built in code, no preview, so the look-check quiz matters here |

Also used, not "libraries": **Minotaur** (Modrinth upload Gradle plugin, world 11; version **verify**).

## 4. Risks and open questions

| Risk | Detail | Plan |
|---|---|---|
| Mappings and API churn across 1.21.x | Mojang names stay, but 1.21.2+ rewrote item rendering and models, 1.21.5+ changed GameTest registration, and NeoForge moves to year-versioned `26.x`. Parchment's last 1.21.1 build is 2024.11.17 (frozen). | Freeze the track at 1.21.1 + one pinned NeoForge. Each lesson names the version it is true for. Add a "what moved in 1.21.x+" read quest per world later, not now. |
| Test speed | Tier B/C tests and GameTests cost 10-60 s, not 3 s. Watcher re-runs on every save would spam. | Debounce watcher to 2 s; run tier A first and stop if red; boss GameTests on explicit "Run boss tests" only. |
| Disk | ~2.4 GB shared + ~0.5 GB per active workspace; Fabric adds its own cache. | Setup quest shows sizes; workspace cleanup opt-in; mention `dev-clean.ps1` style guidance in the Oxide docs. |
| GameTest flakiness | Timing, chunk loading and random spawns. | Fixed empty template, tick-based waits only, seeded random, retry failed GameTest once before reporting. |
| Unverified tooling facts | `unitTest` + `EphemeralTestServerProvider` names, `JUnitLikeTestReporter` in 1.21.1, `@GameTestHolder` in 1.21.1, Architectury API/Loom pins, EMI version. | The measurement spike (below) resolves all of these before any content is written. |
| Copyright of the reference mod | Effortless Building is the user's own mod; do not copy its code into shipped lessons, only its structure. | Quests use original classes. |
| Learner code drift inside a world | A learner's quest-N solution can break quest-N+1. | Contract stubs plus "sync reference solution" (see section 1). |
| Architectury adds a second dev env | World 9 and 11 need the Fabric env too (~1.5 GB more, minutes more). | Tests grade common + NeoForge only; Fabric is built (`assemble`) and left as a manual "run it" step. |
| Fabric overlap | Fabric track is planned in parallel. | This track owns worlds 0-11 for NeoForge; the Fabric capstone ports the world-3 boss, so Ledger (world 3 boss) is the shared artefact. Keep its public shape stable. |
| Rendering untestable | Many worlds' good parts are visual. | Look-check read quests (see section 1). |

### Open questions for the user
1. Should world 9 teach Architectury API as the default and ServiceLoader as the alternative (current plan), or the reverse, since your own mod uses ServiceLoader?
2. Is a ~3 GB first-run download acceptable on the Setup quest, or should the track ask before starting it?
3. Should bosses with `client` marked rows require a screenshot upload? (Not planned; the quiz is the proof.)

### First engine tasks implied
1. 1-hour measurement spike on this machine: cold and warm timings for tiers A/B/C, `runGameTestServer`, `runData`; confirm every **verify** item above.
2. Gradle runner: `pre` tasks, JUnit-XML + GameTest-XML parsing, `workspace = "world"` merge semantics, `replace`.
3. content-check for gradle worlds: build every starter (must compile) and solution (must pass) per world workspace in a temp dir, sharing one `GRADLE_USER_HOME`.

