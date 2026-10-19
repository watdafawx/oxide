# Minecraft: Fabric (1.21.1)

Planning doc for the Fabric section of the Minecraft track. Not content. Design context:
`docs/TRACKS.md` (runners, workspaces), `docs/CONTENT.md` (quest format). The NeoForge section
is planned separately (`docs/tracks/neoforge.md`); this section only teaches what Fabric does
differently and never repeats NeoForge registry/datagen basics beyond that.

Research date: 2026-10-19. Reference project (read-only): `X:\Dev\Minecraft\effortless-building-multi`.

## 1. Toolchain and runner

### Versions to pin (MC 1.21.1)

| piece | pin | source / note |
|---|---|---|
| Minecraft | 1.21.1 | |
| Java | 21 (Temurin 21.0.12 on this machine) | `options.release = 21` |
| Fabric Loader | **0.19.5** | Fabric example mod `1.21.1` branch; meta API lists it as the only newest stable. Reference project is on 0.16.9 (old). |
| Fabric API | **0.116.17+1.21.1** | newest `+1.21.1` on maven.fabricmc.net. Reference: 0.109.0. |
| Loom | **1.19.1** (newest release in maven metadata) or `1.18-SNAPSHOT` (what the example mod uses) | Plugin id is now `net.fabricmc.fabric-loom-remap` in the example mod; the reference project still uses `fabric-loom` + `1.8-SNAPSHOT`. **Verify the id + Gradle version pairing at content build** (see risks). |
| Gradle | whatever Loom pin demands (reference: 8.10) | wrapper per starter |
| Mappings | **Mojang official** (`loom.officialMojangMappings()`), optional Parchment layer | see below |
| ModMenu | 11.0.3 (reference) to 11.0.5 | `com.terraformersmc:modmenu`, maven.terraformersmc.com |
| Cloth Config | 15.0.140 | `me.shedaniel.cloth:cloth-config-fabric`, maven.shedaniel.me |
| MixinExtras | bundled with Fabric Loader (since 0.15) | no extra dependency; `@WrapOperation`, `@Local`, etc. just work |
| JUnit | 5 via `net.fabricmc:fabric-loader-junit:<loader>` | Fabric docs "Automatic Testing" |

**Mappings decision: Mojang, plus Parchment for param names.** Reasons: Fabric dropped Yarn for
26.1+ and calls it unsupported; the Fabric template generator defaults to Mojang mappings (Yarn is
an "Advanced Options" toggle); NeoForge also uses Mojang names, so the two sections share class
names and the cheat sheet (section 3) works. The reference project uses exactly this: layered
`officialMojangMappings()` + `parchment("org.parchmentmc.data:parchment-1.21:2024.11.10@zip")`.
Learners will still meet Yarn names in old tutorials; a read quest in the Mixins world covers
translating (`Entity` vs `class_1297`, `ServerWorld` vs `ServerLevel`).

### Template generator
`fabricmc.net/develop/template` produces a Loom project (Mojmap by default; options: Kotlin,
data generators, split client/main source sets). Oxide does not call it at runtime. Starters are
frozen copies of a generated project (split sources on, datagen on from World 5, Kotlin off), so
setup quest 02 *explains* the generator and shows the options, then the learner edits the
checked-in output.

### Per-world workspace layout

One workspace per world (`workspace = "world"`), per TRACKS.md. Layout of a Fabric world workspace:

```
<workspace>/
  gradle/, gradlew(.bat), settings.gradle, gradle.properties   # pinned versions, from Setup
  build.gradle                                  # loom + fabricApi.configureTests{} + junit
  src/main/java/<pkg>/                          # learner code (common/server)
  src/main/resources/fabric.mod.json, <modid>.mixins.json, assets/, data/
  src/client/java/<pkg>/                        # client-only (splitEnvironmentSourceSets)
  src/test/java/<pkg>/                          # JUnit objectives (Oxide-owned, overwritten each quest)
  src/gametest/java/<pkg>/                      # bosses (GameTest), Oxide-owned
  src/gametest/resources/fabric.mod.json        # registers fabric-gametest entrypoints
```

- `replace = [...]` list per quest: `src/test/**`, `src/gametest/**`, `build.gradle` when a quest
  needs a new dependency (ModMenu in World 4). Learner files under `src/main` and `src/client`
  are never overwritten.
- One shared `GRADLE_USER_HOME` for all worlds: Minecraft jars, decompile/remap caches and
  Loom caches are per-version and shared, so the second Fabric world downloads nothing new.
  A 1.21.1 Loom cache is ~1.5-2 GB, matches the TRACKS.md estimate.
- Each world reuses the *previous* world's mod (World 2 starts from the World 1 boss mod, etc.)
  so the learner builds one growing mod `oxide_fabric` (id `oxidefab`), not 7 throwaways. World 6
  starts from a fixed snapshot so a skipped world cannot block it.

### Testing without a running game (the main design lever)

`fabric-loader-junit` boots Fabric Loader inside the `test` task, so:

1. **Plain logic** (counters, clamps, codecs, config beans): ordinary JUnit, no Minecraft.
2. **Registry-touching code** (items, blocks, components): `@BeforeAll` calls
   `SharedConstants.tryDetectVersion(); Bootstrap.bootStrap();` then asserts on
   `BuiltInRegistries`, `new ItemStack(...)`, etc. Starters ship a `TestBootstrap` helper so
   learners don't write this. Bootstrap adds ~1-2 s to the first test class only.
3. **Mixins in unit tests**: Loader's JUnit setup runs Knot with mixins enabled, so a mixin
   targeting `ItemStack` is applied when the class loads in the test JVM. Test pure behaviour
   (`new ItemStack(Items.X).getMaxStackSize()`), and accessors/invokers by casting to the
   accessor interface. Mixins that need a ticking `ServerLevel`/entity move to GameTest.
   **Needs a spike before content (risk R2).**
4. **Network codecs**: `StreamCodec` round-trip through `RegistryFriendlyByteBuf`: pure JUnit.
5. **Bosses**: `fabric-gametest-api`, a real server with a test structure; assertions are
   `GameTestHelper` calls. Runner target: `fabricApi { configureTests { createSourceSet = true;
   enableGameTests = true; eula = true } }`. Docs say server game tests run as part of `build`;
   TRACKS.md assumes a `runGameTestServer`-style task. **Pin the exact task name in the spike
   (R3).** Parse results from the log (`GameTest ... passed/failed`) into objective statuses;
   each `@GameTest` method name = objective name. 1.21.1 is on the older `FabricGameTest`
   interface *or* the newer `net.fabricmc.fabric.api.gametest.v1.GameTest` annotation depending on
   the Fabric API build; the pinned 0.116.17 decides (R3).
6. **Datagen** quests: grade the *output*. The runner gets an optional `pre = ["runDatagen"]`
   task list per quest; JUnit then asserts on `src/main/generated/**` JSON. Costs ~30-60 s
   once per attempt (full game bootstrap), so these quests set `minutes` honestly.

### What cannot be auto-graded: "run it, look, quiz"

HUD placement, screen layout, item/block textures and models, keybind feel, ModMenu button.
Pattern: the objectives grade what *is* checkable (the Screen class builds N widgets, the
keybinding is registered with the right default, the model JSON exists and references the right
texture), then a final `read`-style step says "run `gradlew runClient`, open X, answer: what
colour is the bar?" as a 1-2 question quiz. Marked **client** in the Runs column. Client-only
quests never gate World progress on the quiz alone (an unreadable screenshot is not a failure).
Optional later: Fabric's `runClientGameTest` can take screenshots in CI, not needed for v1.

Warm-daemon `gradlew test` time: **not measured here** (needs a 2 GB Loom setup; the reference
project sets `org.gradle.daemon=false`, so its numbers would be misleading). Expectation, to be
confirmed in the engine session: first run 2-4 min (Minecraft download + remap), warm re-runs
with a live daemon and configuration cache 3-8 s for a small mod (JIT cold test JVM + Knot
startup dominate, not compilation). First engine task: time it and set `GRADLE_OPTS`/daemon
policy accordingly.

## 2. Worlds

Columns: **objectives** are JUnit/GameTest names (suffix-matched). **Runs**: `junit` =
`gradlew test`; `junit+boot` = needs Bootstrap; `gametest`; `datagen` = needs `runDatagen` first;
`client` = ends with a run-it-look-quiz. XP follows CONTENT.md (exercise 30-80, read 15, boss
300-500). Quick wins (lower XP/minutes) are marked ⚡. The Setup world has no boss (it gets a
"it runs" read as finale); every other world ends in a boss.

Folder names: `content/tracks/fabric/worlds/NN-slug/NN-quest/`. Quest ids `fabric/<world>/<quest>`.
All quests below `requires` the previous quest unless stated.

### World 0: Setup (`00-setup`), 6 quests, first one under 5 min

| slug | idea | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-toolchain-check | JDK 21, wrapper, what gets downloaded (2 GB) | quiz: 2 | 3 | 15 | read | - |
| 02-generate-project | Template generator options; fix a generated project's wrong ids | `mod_json_has_id`, `entrypoint_class_exists` | 5 | 30 | exercise ⚡ | junit |
| 03-fabric-mod-json | `depends`, `environment`, entrypoints, version expansion | `depends_pins_minecraft_1_21_1`, `requires_java_21`, `lists_main_and_client_entrypoint` | 8 | 40 | exercise | junit |
| 04-gradle-anatomy | Loom, `modImplementation`, mappings, split source sets, `runClient` | quiz: 3 | 4 | 15 | read | - |
| 05-hello-initializer | `ModInitializer`, `LOGGER`, a testable greeting | `greeting_names_the_mod`, `initializer_implements_mod_initializer` | 6 | 35 | exercise | junit |
| 06-run-it-once | `gradlew runClient`, find your mod in the mod list, read the log | quiz: 2 | 5 | 15 | read ⚡ | client |

### World 1: Registries & Fabric API (`01-registries`), 9 quests

Only the Fabric differences: plain `Registry.register`, no deferred registration, static init
order, Fabric API helpers.

| slug | idea | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-registry-model | `Registry.register` at init, no DeferredRegister, why ordering matters | quiz: 2 | 3 | 15 | read | - |
| 02-register-item | Register an item with `Item.Properties` | `item_registered_in_mod_namespace`, `item_stacks_to_16` | 8 | 40 | exercise | junit+boot |
| 03-register-block-and-item | Block + BlockItem pairing, hardness, drops itself | `block_registered`, `block_item_registered`, `block_hardness_is_set` | 10 | 50 | exercise | junit+boot |
| 04-creative-tab | `ItemGroupEvents.modifyEntriesEvent` into a vanilla tab, then your own tab | `item_in_tools_tab`, `custom_tab_registered` | 6 | 35 | exercise ⚡ | junit+boot |
| 05-registry-helpers | `FuelRegistry`, `CompostingChanceRegistry`, `StrippableBlockRegistry`, `FlammableBlockRegistry` | `fuel_time_is_200`, `compost_chance_set` | 6 | 30 | exercise ⚡ | junit+boot |
| 06-item-components | 1.21 data components: `FoodProperties`, custom component with a Codec | `food_nutrition_is_4`, `custom_component_roundtrips_codec` | 10 | 55 | exercise | junit+boot |
| 07-block-entity | `BlockEntityType`, save/load NBT, a server ticker counting ticks | `block_entity_type_registered`, `tick_increments_counter`, `save_load_keeps_counter` | 15 | 70 | exercise | junit+boot |
| 08-data-attachments | Fabric Attachment API (`AttachmentRegistry`, persistent + copy-on-death) | `attachment_registered_persistent`, `default_value_is_zero`, `copy_on_respawn_enabled` | 10 | 60 | exercise | junit+boot |
| 09-boss-rune-forge | A craftable block that eats a fuel item, counts uses in a block entity, drops a rune item | `forge_block_registered`, `rune_item_registered`, `forge_consumes_fuel`, `forge_outputs_rune`, `counter_persists` (GameTest: `forge_works_in_world`) | 45 | 350 | boss | junit+boot, gametest |

### World 2: Mixins (`02-mixins`), 12 quests, the deepest world

What Fabric modders actually struggle with: where to inject, reading obfuscated-ish names,
arguments and locals, conflict, and error messages. Every mixin quest has a **"why not an
event?"** line, because Fabric API events usually beat a mixin and learners should reach for
those first (World 3 returns to this).

| slug | idea | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-what-is-a-mixin | Bytecode merge at class load, when to use (no event exists), when not to; Yarn vs Mojmap names | quiz: 3 | 5 | 15 | read | - |
| 02-config-and-first-mixin | `<modid>.mixins.json`: `package`, `mixins`/`client`/`server`, `required`, `injectors.defaultRequire`, registered in `fabric.mod.json`; an empty `@Mixin(ItemStack.class)` that loads | `mixin_config_lists_class`, `package_matches_mixin_class`, `mixin_class_is_abstract_or_interface_safe` | 8 | 40 | exercise | junit+boot |
| 03-inject-head-cancel | `@Inject(at=@At("HEAD"), cancellable=true)`, `CallbackInfo.cancel()`. Example: stop `ItemStack` damage when a flag is set | `damage_blocked_when_flag_set`, `damage_normal_otherwise` | 10 | 60 | exercise | junit+boot |
| 04-inject-return | `@At("RETURN")`, `CallbackInfoReturnable.setReturnValue`; change a returned value | `max_stack_size_doubled`, `other_items_unchanged` | 10 | 55 | exercise | junit+boot |
| 05-accessor | `@Accessor` on a private/protected field (getter + setter), cast to the accessor interface | `reads_private_field`, `writes_private_field` | 5 | 30 | exercise ⚡ | junit+boot |
| 06-invoker | `@Invoker` for a private/protected method; invoker vs accessor vs access widener (`.accesswidener` file in `loom { accessWidenerPath }`) | `invokes_protected_method`, `widener_lists_class` | 8 | 45 | exercise | junit+boot |
| 07-shadow-unique-duck | `@Shadow`, `@Unique` fields/methods, interface injection ("duck typing") to add state to vanilla classes; why `@Unique` prefixes names | `shadow_reads_target_field`, `unique_state_per_instance`, `duck_interface_implemented` | 12 | 65 | exercise | junit+boot |
| 08-modify-arg-constant | `@ModifyArg`, `@ModifyConstant`, `@ModifyVariable`: change one value without cancelling. | `arg_is_modified`, `constant_replaced` | 6 | 30 | exercise ⚡ | junit+boot |
| 09-local-capture | MixinExtras `@Local` to read locals, `@Share` between injectors; why raw `locals = CAPTURE_FAILHARD` is brittle | `local_value_read`, `shared_value_visible_to_second_injector` | 12 | 70 | exercise | junit+boot |
| 10-wrapoperation-family | `@Redirect` (and why it conflicts) vs `@WrapOperation`, `@WrapWithCondition`, `@ModifyExpressionValue`, `@ModifyReceiver`; two mixins from two "mods" on the same call must both work | `wrap_operation_calls_original`, `wrap_with_condition_skips_call`, `modify_expression_value_chains`, `two_mixins_coexist` | 15 | 80 | exercise | junit+boot |
| 11-debug-mixin-errors | Read a real `InvalidInjectionException`, `MixinTransformerError`; `-Dmixin.debug.export=true`, `mixin.debug.verbose`; priority and `require`/`expect`/`allow` | quiz: 3 | 5 | 15 | read ⚡ | - |
| 12-boss-sleepless-world | Combine accessor + invoker + inject + WrapOperation into one feature ("Sleep Anywhere": beds work in any dimension and keep spawn). Staged: first objective (config loads) passes early. | `mixin_config_registered`, `accessor_reads_sleep_timer`, `inject_head_allows_sleep`, `wrap_operation_keeps_spawn`, `local_capture_logs_reason`, `no_conflict_with_second_mod` (GameTest: `sleep_in_nether_works`) | 60 | 450 | boss | junit+boot, gametest |

Illustrative snippets for the lessons (targets are real in 1.21.1 Mojmap but **content sessions
must re-verify method names/descriptors against the 1.21.1 sources**, mixins break on typos):

```java
// 03/04 inject: ItemStack#getMaxStackSize
@Mixin(ItemStack.class)
abstract class ItemStackMixin {
    @Inject(method = "getMaxStackSize", at = @At("RETURN"), cancellable = true)
    private void oxide$bigStacks(CallbackInfoReturnable<Integer> cir) {
        cir.setReturnValue(cir.getReturnValue() * 2);
    }
}

// 05 accessor + 06 invoker
@Mixin(Mob.class)
public interface MobAccessor { @Accessor("goalSelector") GoalSelector oxide$goalSelector(); }

@Mixin(LivingEntity.class)
public interface LivingEntityInvoker { @Invoker("jumpFromGround") void oxide$jump(); }

// 10 wrap operation (MixinExtras): wrap a call instead of replacing it
@WrapOperation(method = "hurt", at = @At(value = "INVOKE", target = "..."))
private boolean oxide$wrap(Entity self, DamageSource src, float amount, Operation<Boolean> original) {
    return original.call(self, src, amount * 0.5f);
}
```

Naming convention for learners: prefix non-private members `oxide$` (the mixin docs' `@Unique`
convention; the reference project uses plain names).

### World 3: Events & networking (`03-events-networking`), 11 quests

| slug | idea | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-event-model | Fabric API `Event<T>`, callbacks return `InteractionResult` (PASS / SUCCESS / FAIL), phases and ordering | quiz: 2 | 4 | 15 | read | - |
| 02-server-tick-counter | `ServerTickEvents.END_SERVER_TICK`, a tick counter class | `counts_every_tick`, `fires_action_every_n_ticks` | 8 | 40 | exercise | junit |
| 03-use-block-callback | `UseBlockCallback`: right-click rules, spectator/hand checks | `ignores_spectators`, `blocks_use_with_stick`, `passes_otherwise` | 10 | 50 | exercise | junit+boot |
| 04-damage-events | `ServerLivingEntityEvents.ALLOW_DAMAGE`, `AttackEntityCallback` | `cancels_fall_damage_for_marked_player`, `allows_other_damage` | 6 | 30 | exercise ⚡ | junit+boot |
| 05-lifecycle-and-join | `ServerLifecycleEvents`, `ServerPlayConnectionEvents.JOIN`; welcome message once per player | `welcomes_first_join_only`, `server_started_hook_registered` | 8 | 40 | exercise | junit+boot |
| 06-custom-event | `EventFactory.createArrayBacked`, a callback interface, fired from your code (and later from a mixin, link to World 2) | `invoker_calls_all_listeners`, `first_non_pass_wins`, `fail_stops_chain` | 12 | 60 | exercise | junit |
| 07-payload-codec | `CustomPacketPayload` record, `Type`, `StreamCodec.composite` | `codec_roundtrips`, `type_id_in_mod_namespace` | 10 | 55 | exercise | junit+boot |
| 08-validate-on-server | Never trust a C2S payload: range, null, rate limits (pure validator) | `rejects_out_of_range_target`, `accepts_valid`, `rate_limit_blocks_spam` | 6 | 35 | exercise ⚡ | junit |
| 09-register-and-send | `PayloadTypeRegistry.playC2S()/playS2C()`, `ServerPlayNetworking.registerGlobalReceiver`, `ClientPlayNetworking.send`; where to register (common init) | `both_directions_registered`, `server_handler_registered` | 10 | 55 | exercise | junit+boot |
| 10-commands | `CommandRegistrationCallback`, Brigadier registration only (Brigadier itself lives in the NeoForge section) | `command_registered`, `command_returns_count` | 6 | 35 | exercise ⚡ | junit+boot |
| 11-boss-combo-counter | Server-authoritative combo counter: client sends hit, server validates + counts, S2C syncs; resets after 3 s | `payload_codec_roundtrips`, `server_rejects_spoof`, `combo_increments`, `combo_resets_after_timeout`, `sync_payload_sent` (GameTest: `combo_syncs_to_player`) | 50 | 400 | boss | junit+boot, gametest |

### World 4: Client (`04-client`), 8 quests

| slug | idea | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-client-split | `ClientModInitializer`, `splitEnvironmentSourceSets`, `@Environment(EnvType.CLIENT)`, crash on dedicated server if mixed up | quiz: 2 | 4 | 15 | read | - |
| 02-keybinding | `KeyBindingHelper.registerKeyBinding`, `ClientTickEvents`, toggle logic | `registers_with_default_key`, `toggle_flips_state` | 8 | 40 | exercise | junit+boot |
| 03-hud-element | `HudRenderCallback`, draw text/bar; formatted value | `formats_percentage`, `hud_callback_registered` | 10 | 55 | exercise | junit, client |
| 04-open-a-screen | A basic `Screen` with a button and a label | `screen_has_one_button`, `button_runs_action` | 8 | 45 | exercise | junit+boot, client |
| 05-lang-and-textures | `assets/<modid>/lang/en_us.json`, item model + texture JSON, `BlockRenderLayerMap` for glass | `lang_keys_present`, `model_references_texture`, `texture_file_exists` | 6 | 30 | exercise ⚡ | junit, client |
| 06-modmenu | `ModMenuApi#getModConfigScreenFactory`, `modmenu` entrypoint in `fabric.mod.json` | `entrypoint_declared`, `factory_returns_screen` | 6 | 35 | exercise ⚡ | junit+boot |
| 07-cloth-config | Config bean (GSON/AutoConfig), clamp + load/save, Cloth `ConfigBuilder` screen | `config_roundtrips_json`, `values_clamped`, `screen_has_expected_entries` | 15 | 70 | exercise | junit+boot, client |
| 08-boss-build-ruler | Keybind toggles a ruler HUD showing block distance, with a Cloth/ModMenu config screen for colour and unit | `keybind_toggles_ruler`, `distance_formats_blocks`, `config_clamps_scale`, `config_screen_builds`, `modmenu_entrypoint_declared` | 50 | 350 | boss | junit+boot, client |

### World 5: Data & datagen (`05-datagen`), 7 quests

NeoForge's datagen concepts are assumed (or skipped by learners who did that section); here only
Fabric's setup and entrypoint differ, then each provider quickly.

| slug | idea | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-datagen-setup | `fabricApi { configureDataGeneration() }`, `DataGeneratorEntrypoint`, `fabric-datagen` source set, `runDatagen`, generated resources folder | `entrypoint_declared`, `generated_dir_on_resource_path` | 8 | 40 | exercise | junit, datagen |
| 02-lang-provider | `FabricLanguageProvider.generateTranslations` | `lang_has_item_name`, `lang_has_tab_name` | 5 | 30 | exercise ⚡ | datagen |
| 03-model-provider | `FabricModelProvider`: item generated, block cube-all | `item_model_generated`, `block_model_cube_all`, `blockstate_exists` | 12 | 55 | exercise | datagen |
| 04-recipe-provider | `FabricRecipeProvider`: shaped + shapeless, unlocked-by criteria | `shaped_recipe_exists`, `shapeless_recipe_exists`, `recipes_have_unlock_criteria` | 12 | 55 | exercise | datagen |
| 05-loot-provider | `FabricBlockLootTableProvider`: drop self, silk-touch ore | `block_drops_self`, `ore_has_silk_touch_branch` | 12 | 55 | exercise | datagen |
| 06-tag-provider | `FabricTagProvider.ItemTagProvider`/`BlockTagProvider`, mineable + `needs_iron_tool` | `pickaxe_mineable_tag`, `needs_iron_tool_tag` | 6 | 35 | exercise ⚡ | datagen |
| 07-boss-datagen-the-mod | Regenerate everything for the Rune Forge from World 1: no hand-written JSON left | `lang_complete`, `models_complete`, `recipe_for_forge`, `loot_for_forge`, `tags_complete`, `no_handwritten_data_files` | 50 | 350 | boss | datagen |

### World 6: Capstone (`06-capstone`), 8 quests

Requires the NeoForge boss feature (id TBD with the NeoForge planning session, see open
questions). Starts from a snapshot of "a NeoForge boss feature" source in the starter so
learners without that section can still do it.

| slug | idea | objectives | min | XP | kind | runs |
|---|---|---|---|---|---|---|
| 01-read-the-neoforge-boss | Map every NeoForge API used in the boss to its Fabric twin (use section 3) | quiz: 3 | 5 | 15 | read | - |
| 02-port-registration | `DeferredRegister` to `Registry.register`, `@Mod` to `ModInitializer` | `boss_item_registered`, `boss_entity_type_registered` | 12 | 60 | exercise | junit+boot |
| 03-port-events | `@SubscribeEvent` handlers to Fabric callbacks | `damage_event_hooked`, `tick_event_hooked`, `behaviour_matches_neoforge_reference` | 12 | 60 | exercise | junit+boot |
| 04-port-networking | `RegisterPayloadHandlersEvent` to `PayloadTypeRegistry` + receivers | `payloads_registered_both_ways`, `codec_matches_reference_bytes` | 15 | 70 | exercise | junit+boot |
| 05-mixin-for-the-gap | NeoForge has an event Fabric lacks (e.g. a bake/render hook); fill it with a mixin or an event of your own | `custom_event_fired_from_mixin`, `no_neoforge_import_left` | 15 | 80 | exercise | junit+boot |
| 06-common-module | Split into `common` + `fabric`; platform service via `ServiceLoader` (what the reference project does) vs `@ExpectPlatform` | `common_has_no_loader_imports`, `platform_service_resolves`, `fabric_module_depends_on_common` | 15 | 70 | exercise | junit |
| 07-architectury-api | Replace the hand-rolled service layer with Architectury API (registries, events, networking) | `registers_via_architectury`, `event_via_architectury`, `network_via_architectury` | 15 | 80 | exercise | junit+boot |
| 08-boss-ship-it | Both loaders build and pass the same GameTest; produce a merged jar (Forgix-style or two jars) | `fabric_jar_builds`, `neoforge_jar_builds`, `common_tests_pass`, `fabric_gametest_passes`, `neoforge_gametest_passes`, `metadata_ids_match` | 60 | 500 | boss | junit, gametest (both loaders) |

### Totals (computed from the tables above)

| world | quests | XP |
|---|---|---|
| 00 Setup | 6 | 150 |
| 01 Registries | 9 | 705 |
| 02 Mixins | 12 | 955 |
| 03 Events & networking | 11 | 815 |
| 04 Client | 8 | 640 |
| 05 Datagen | 7 | 620 |
| 06 Capstone | 8 | 935 |
| **Total** | **61** | **4820** |

## 3. Fabric vs NeoForge cheat sheet (1.21.1)

Same Mojang class names on both, so only the plumbing differs.

| task | Fabric | NeoForge |
|---|---|---|
| mod entry | `fabric.mod.json` entrypoints: `ModInitializer`, `ClientModInitializer` | `@Mod("id")` class, `neoforge.mods.toml` |
| register an item | `Registry.register(BuiltInRegistries.ITEM, ResourceLocation.fromNamespaceAndPath(ID, "gem"), new Item(props))` at init | `DeferredRegister.Items ITEMS`; `ITEMS.registerSimpleItem("gem", props)`; `ITEMS.register(modBus)` |
| add to creative tab | `ItemGroupEvents.modifyEntriesEvent(KEY).register(e -> e.accept(item))` | `BuildCreativeModeTabContentsEvent` on the mod bus |
| listen to an event | `UseBlockCallback.EVENT.register((player, level, hand, hit) -> InteractionResult.PASS)` | `@SubscribeEvent void on(PlayerInteractEvent.RightClickBlock e)` on the game bus |
| server tick | `ServerTickEvents.END_SERVER_TICK.register(server -> ...)` | `ServerTickEvent.Post` |
| cancel a result | return `FAIL` / `SUCCESS` | `event.setCanceled(true)` |
| custom event | `EventFactory.createArrayBacked(...)` | subclass `Event`, post on bus |
| send a packet | record `CustomPacketPayload` + `PayloadTypeRegistry.playC2S().register(TYPE, CODEC)`; `ClientPlayNetworking.send(p)`; receive with `ServerPlayNetworking.registerGlobalReceiver(TYPE, (p, ctx) -> ...)` | record `CustomPacketPayload`; `RegisterPayloadHandlersEvent` + `registrar.playToServer(TYPE, CODEC, handler)`; `PacketDistributor.sendToServer(p)` |
| add a config screen | ModMenu `ModMenuApi#getModConfigScreenFactory` + Cloth Config `ConfigBuilder`/AutoConfig | `ModConfigSpec` + `container.registerExtensionPoint(IConfigScreenFactory.class, ...)` or built-in `ConfigurationScreen` |
| player/world data | `AttachmentRegistry` (Fabric API) | `AttachmentType` (data attachments) |
| keybind | `KeyBindingHelper.registerKeyBinding` | `RegisterKeyMappingsEvent` |
| client only code | separate `client` source set, `@Environment(EnvType.CLIENT)` | `Dist.CLIENT` / `@EventBusSubscriber(value = Dist.CLIENT)` |
| private access | access widener (`.accesswidener`) or accessor mixin | access transformer (`accesstransformer.cfg`) or accessor mixin |
| mixin config | listed in `fabric.mod.json` `"mixins"` | listed in `neoforge.mods.toml` `[[mixins]]` |
| datagen | `DataGeneratorEntrypoint` + `Fabric*Provider` | `GatherDataEvent` + vanilla/NeoForge providers |
| run configs | `loom { runs { client { ... } } }` | `neoForge { runs { client { ... } } }` (ModDevGradle) |
| build plugin | Fabric Loom | NeoForge ModDevGradle (`net.neoforged.moddev`) |

Each World 3/4 quest ends with a one-line "In NeoForge this is ..." box taken from this table.

## 4. Risks and open questions

**Risks**
- **R1 Loom/Gradle pairing.** Reference uses Loom `1.8-SNAPSHOT` + Gradle 8.10 + plugin id
  `fabric-loom`; the current example mod uses plugin `net.fabricmc.fabric-loom-remap` with
  `1.18-SNAPSHOT`; newest release is 1.19.1. Snapshots are not reproducible. Spike: generate a
  1.21.1 project with the pinned versions, check `gradlew test` works, then freeze one pairing
  (preferably a *release* Loom) and record the Gradle version.
- **R2 Mixins under JUnit.** Plan assumes fabric-loader-junit applies mixins to Minecraft
  classes in the test JVM. Verify with one `ItemStack` mixin before writing World 2; fallback:
  move most mixin objectives to GameTest (slower, ~20-30 s per run) and keep only
  accessor/invoker/config tests in JUnit.
- **R3 GameTest task and API flavour.** Docs say server game tests run under `build`, TRACKS.md
  assumes a dedicated task, and 1.21.1 may use either the older `FabricGameTest` interface or the
  v1 annotation. Pin the task name and parse format in the spike; if no standalone task exists,
  use a Loom run config `gametest` and `-Dfabric-api.gametest`.
- **R4 Test speed.** Warm re-run time unmeasured (section 1). If over ~10 s, objectives tick too
  slowly for the "save and watch" loop; fall back to running only the quest's test classes
  (`--tests`) and keeping Bootstrap lazy.
- **R5 Mixin brittleness.** Method names and descriptors in snippets are from memory of 1.21.1
  Mojmap; every mixin quest solution needs a CI build that proves it applies (content-check runs
  the test, which loads the class).
- **R6 Datagen cost.** 30-60 s per attempt (`pre = ["runDatagen"]`). Acceptable for 7 quests,
  but it breaks "instant tick". Needs the pre-task runner option in the engine.
- **R7 Client quests.** HUD/screen/texture quality is unverifiable; the quiz is honour-based.
  Don't gate progress on it.
- **R8 Dependency fetches.** ModMenu/Cloth/Parchment come from third-party Maven repos; offline
  or flaky network breaks World 4/5. Pre-warm in the Setup read.
- **R9 Disk.** A shared Gradle home plus shared Loom cache: ~2 GB first Fabric world, then small.
  Per-world `.gradle` and `build` dirs add a few hundred MB; Oxide's cleanup should remove
  workspaces of finished worlds only on user request.
- **R10 Version drift.** Fabric docs now target 26.x; most tutorials have 1.21.11/26.x APIs.
  Lessons must be written against 1.21.1 docs only (`docs.fabricmc.net/1.21.1/...`).

**Open questions for the main session**
1. NeoForge boss id for the capstone: the starter needs a fixed NeoForge boss feature to port.
   Which one does the NeoForge planning session choose?
2. Setup world with no boss: OK (CONTENT.md says "6-12 quests + boss")?
3. Runner needs: optional `pre = [tasks]` per quest (datagen), GameTest result parsing from log,
   a TestBootstrap helper in starters. Accepted?
4. Does Oxide ship Parchment (extra repo) or plain Mojmap only? Plain is simpler; Parchment makes
   parameter names readable (`level`, `pos`) in decompiled sources. Recommendation: plain
   Mojmap in v1, mention Parchment in the Setup read.
5. Capstone Architectury part: do learners keep Fabric working while NeoForge builds, or is the
   NeoForge half provided as a ready subproject? Plan assumes the NeoForge half is provided.
