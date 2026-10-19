# Tracks: Oxide beyond Rust

Oxide started as a Rust course. Tracks add Java (and its frameworks), Minecraft modding
(NeoForge 1.21.1+, Architectury, Fabric) and Lua (core, then Factorio). Same loop everywhere:
one small quest, edit in your editor, save, objectives tick, XP.

Status: design. Per-track curricula are planned in `docs/tracks/*.md`; nothing here is built yet.

## Content layout

```
content/tracks/<track>/track.toml          title, blurb, runner, toolchain notes
content/tracks/<track>/worlds/<NN-world>/<NN-quest>/   quest.toml, lesson.md, starter/, solution/
```

Rust moves from `content/worlds` to `content/tracks/rust/worlds` once the current content sessions
finish (ids become `rust/01-foundations/03-functions`; no learner progress exists yet).
Quest ids: `<track>/<world>/<quest>`. `requires` may cross tracks (Minecraft requires Java basics).

## Runners (engine: `oxide-core/src/grader/`)

`quest.toml` gains `runner = "cargo" | "gradle" | "lua"` (default from `track.toml`). Every runner
returns the same `CheckResult`: compiled?, one status per objective, diagnostics with file:line,
raw tail. Objectives are still named test functions.

| runner | command | objectives from | notes |
|---|---|---|---|
| `cargo` | `cargo test --message-format=json-render-diagnostics` | `test name ... ok` lines | as today |
| `gradle` | `gradlew.bat test --console=plain` with `JAVA_HOME` = detected JDK 21 | JUnit XML in `build/test-results/test/TEST-*.xml` (`<testcase name>` + `<failure>` text) | compiler errors parsed from stdout (`File.java:12: error: ...`). Shared `GRADLE_USER_HOME` and a warm daemon: re-runs 2-5 s, first run ~1 min. |
| `lua` | in-process Lua 5.2 (`mlua`, vendored, no install) runs `tests.lua` under `oxide/test.lua` | harness prints `test name ... ok/FAILED` (cargo's format, same parser) | Lua 5.2 because Factorio is 5.2. Factorio quests get `mock_factorio.lua` on the package path: `data:extend`, `data.raw`, `script.on_event`, `defines`, `game.print` capture, `global`/`storage`, simple surface/entity stubs. |

Minecraft specifics: a NeoForge/Fabric dev environment is ~2 GB and minutes to set up, so
those worlds use **one workspace per world** (`workspace = "world"` in quest.toml): the first quest
copies the MDK, later quests merge their `starter/` files into it (never overwriting learner
files that already exist, except files listed in `replace = [...]`). Logic is unit-tested with
JUnit against mod classes (no Minecraft needed); bosses add a NeoForge GameTest / Fabric
`fabric-gametest-api` run (`gradlew runGameTestServer`), parsed from the log into objectives.
Architectury projects: tests run in `common` and per loader.

Watcher: extensions per runner (`rs,toml` / `java,kt,kts,gradle,json,toml` / `lua`).

## UI

- A **track pill** in the top bar (Rust / Java / Minecraft / Lua). Home shows the next quest of
  the active track; Map is per track; Profile XP is global, with a per-track bar.
- A track's first quest is a "Setup" read-quest that checks the toolchain (JDK found, Gradle
  wrapper runs, Factorio path optional) and shows what will be downloaded.
- `requires` across tracks shows as "needs Java › Basics" on the locked card.

## Track skeletons

The planning sessions turn each into 8-12 quests per world with objectives, minutes and XP.

**Java** (`java`, runner gradle, JUnit 5)
0 Setup · 1 Basics (types, control flow, methods, strings) · 2 OOP (classes, interfaces,
inheritance, records, enums, sealed) · 3 Collections & Streams · 4 Exceptions & IO · 5 Generics &
Optional · 6 Build & Test (Gradle, JUnit, Mockito, AssertJ) · 7 Concurrency (threads, executors,
CompletableFuture, virtual threads) · 8 Spring Boot (DI, REST, validation, JPA + H2, tests with
MockMvc) · 9 Capstone: a REST service + CLI client. Frameworks beyond Spring (JavaFX, Quarkus)
as optional side worlds.

**Minecraft: NeoForge 1.21.1+** (`neoforge`, runner gradle, requires java/02-oop)
0 Setup (MDK, run client once) · 1 Registries (items, blocks, block entities, creative tab) ·
2 Data (datagen: models, lang, recipes, loot, tags) · 3 Events & capabilities (NeoForge event
bus, attachments, item handlers) · 4 Interaction (right-click, menus/screens, containers) ·
5 Networking (payloads, client↔server sync) · 6 Entities (custom mob, AI goals, renderer) ·
7 World (worldgen features, structures, dimensions intro) · 8 Commands & config (Brigadier,
NeoForge config spec, Cloth Config) · 9 Architectury (common/neoforge/fabric layout,
`@ExpectPlatform`, Architectury API registries/events/networking) · 10 Ecosystem (JEI/EMI plugin,
Curios/Accessories, GeckoLib model + animation, Jade tooltips, KubeJS hook) · 11 Capstone: a
multi-loader mod, built and published to Modrinth. Each world's boss is a real playable feature.

**Minecraft: Fabric** (`fabric`, runner gradle, requires java/02-oop; shares worlds 1-2 ideas
with NeoForge but its own API): 0 Setup (Loom) · 1 Registries & Fabric API · 2 Mixins (what,
when, injects, accessors; the thing Fabric modders must learn) · 3 Events & networking (Fabric
API events, packets) · 4 Client (rendering, HUD, keybinds, ModMenu + Cloth Config) · 5 Data &
datagen · 6 Capstone: port a NeoForge world-boss to Fabric, then unify with Architectury.

**Lua** (`lua`, runner lua)
1 Lua core (values, tables, functions, closures, iterators, metatables, modules, coroutines,
error handling, string patterns) · 2 Factorio data stage (prototypes, items, recipes,
technologies, `data.raw` edits, graphics paths) · 3 Factorio control stage (events, `storage`,
entities, surfaces, on_tick budgeting, GUI, remote interfaces, migrations) · 4 Factorio craft
(performance, settings, locale, multiplayer safety, mod portal) · 5 Capstone: a real mod
loaded in Factorio (graded by the mock; a manual "load it in the game" step is the reward).

## Toolchains on this machine (2026-10-19)

JDK 21: `C:\Program Files\Eclipse Adoptium\jdk-21.0.12.101-hotspot` (JAVA_HOME points at a JRE 17;
Oxide sets JAVA_HOME per gradle run). Gradle: wrapper per starter (downloads itself). Lua: none
installed, embedded in the engine. Factorio: not in the default paths; optional, for playing the
result. Minecraft reference project: `X:\Dev\Minecraft\effortless-building-multi` (Architectury,
NeoForge + Fabric 1.21.1).

## Order of work

1. Planning sessions write `docs/tracks/{java,neoforge,fabric,lua}.md` (research current APIs,
   verify versions, flag what cannot be auto-graded).
2. Engine: tracks + runner trait + gradle runner + lua runner + per-world workspaces;
   content-check learns each runner. Move Rust under `content/tracks/rust`.
3. UI: track pill, per-track map, setup quests.
4. Content sessions per world, Java first (Minecraft depends on it), then Lua, then Minecraft.
