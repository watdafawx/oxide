# Track plan: Java 25 (delta track `java-25`)

Status: plan (2026-10-19). Runner `gradle`, JUnit, **JDK 25 (Temurin 25.0.4)**, Spring Boot **4.1.1**. Design context in
`docs/TRACKS.md` ("Versions"), quest format in `docs/CONTENT.md`, the course this builds on in `docs/tracks/java.md`
(JDK 21, Boot 3.5.16). The `java` content stays untouched; this track only adds what changed. Nothing is built yet.

**Totals:** 5 worlds, 33 quests (4 bosses, 6 reads), 2,995 XP, ~493 min (~8 h).

| world | quests | read | boss | minutes | XP |
|---|---|---|---|---|---|
| 00 Setup | 4 | 2 | 0 | 19 | 100 |
| 01 Language 22-25 | 7 | 1 | 1 | 87 | 625 |
| 02 Library and runtime | 6 | 1 | 1 | 92 | 640 |
| 03 Spring Boot 4 | 10 | 1 | 1 | 156 | 875 |
| 04 Capstone: habits on 25 | 6 | 1 | 1 | 139 | 755 |
| **total** | **33** | 6 | 4 | **493** | **2,995** |

Sources checked 2026-10-19: Maven Central (Boot BOM 4.1.1), services.gradle.org, `winget search`, openjdk.org JEPs,
Spring Boot 4.0 migration guide and release notes, Spring Framework 7.0.9 reference.

---

## 1. Toolchain

### Pins

| thing | pin | notes |
|---|---|---|
| JDK | **Temurin 25.0.4** (winget `EclipseAdoptium.Temurin.25.JDK`) | LTS. winget also lists `...25.JRE`: do not use, Gradle needs `javac`. |
| Gradle wrapper | **9.8.1** (same as `java`) | JDK 25 needs Gradle **9.1.0+** (Gradle compatibility table). 9.8.1 is current stable, so the daemon is shared with `java` starters only if the JDK also matches (it will not: separate daemons, see below). |
| JUnit | **6.0.3** via `org.junit:junit-bom` | Same as the Boot 4.1.1 BOM. `java` stays on 5.14.4. 6.x is Java 17+; test XML format and `<testcase name>` rules are unchanged, so the engine needs no change. Latest 6.x on Central is 6.1.3: do not use, Boot manages 6.0.3 and mixing causes BOM conflicts. |
| AssertJ | 3.27.7 | same as Boot BOM |
| Mockito | 5.23.0 | Boot BOM. Standalone quests: keep the `-javaagent` line from `java` (R6 there still applies on 25). |
| Spring Boot | **4.1.1** (4.2.0 is at M2; 4.0.8 is the last 4.0) | Gradle plugin `org.springframework.boot` 4.1.1. Keep `io.spring.dependency-management` **1.1.7** (what Initializr emits); verify it still behaves in the measurement spike (Q3). |
| Boot 4.1.1 BOM | Spring Framework **7.0.9**, Spring Data 2026.0.1, Hibernate **7.4.5**, Hibernate Validator 9.1.3, Tomcat **11.0.24**, Jackson **3.1.5** (+ Jackson 2.21.5 BOM kept for the compat module), H2 2.4.240, jspecify 1.0.1, Jakarta Servlet 6.1 / Persistence 3.2 | |

Boot 4 and Framework 7 run on Java 17+, so worlds 03-04 do not *need* JDK 25. They run on 25 because this is one track with
one toolchain, and 25 is what current Minecraft tooling will want.

### Install path on Windows

1. Setup quest 00-01 detects JDK 25 (`C:\Program Files\Eclipse Adoptium\jdk-25.0.*-hotspot`, then `JAVA_HOME`, then PATH).
2. If missing: shows "Temurin 25, ~190 MB download, installs beside JDK 21, changes nothing about 21", **asks**, then runs
   `winget install --id EclipseAdoptium.Temurin.25.JDK -e` (admin prompt from the MSI; no `--silent` so the learner sees it).
3. `track.toml`: `jdk = 25`. Engine sets `JAVA_HOME` to the detected 25 for every `java-25` Gradle run. The `java` track keeps `jdk = 21`.
   The machine already has `jdk-21.0.12.101-hotspot` and a JRE 17 as `JAVA_HOME`: never rely on the ambient one.
4. If winget is missing or declined: quest shows the manual download link (adoptium.net, Windows x64 MSI) and re-checks on button press.

### Starter `build.gradle.kts` vs the Java 21 starters

Plain worlds (00-02):

```kotlin
plugins { java }

repositories { mavenCentral() }

java {
    toolchain { languageVersion = JavaLanguageVersion.of(25) }   // NEW: fails fast if run on the wrong JDK
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:6.0.3"))     // 5.14.4 in `java`
    testImplementation("org.junit.jupiter:junit-jupiter")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher") // still required on Gradle 9
    testImplementation("org.assertj:assertj-core:3.27.7")
}

tasks.test {
    useJUnitPlatform()
    testLogging { events("failed"); exceptionFormat = org.gradle.api.tasks.testing.logging.TestExceptionFormat.FULL }
}
```

Differences from `java.md`: (a) the toolchain block (the Java 21 starters omit it because they have no resolver plugin;
here it only *checks*: Gradle auto-detects the Adoptium install, and with no `foojay-resolver` plugin it never downloads, so a missing
JDK 25 gives "No matching toolchains found", which the engine maps to "Run the Setup quest"); (b) JUnit 6.0.3; (c) nothing else.
`junit-platform-launcher` stays mandatory (Gradle 9, same message as in `java`).

Spring worlds (03-04): add `id("org.springframework.boot") version "4.1.1"` and `id("io.spring.dependency-management") version "1.1.7"`,
drop the JUnit BOM lines (Boot manages 6.0.3), keep `junit-platform-launcher`. Starters are **modular** (world 03).
Quests in world 03 start from different dependency lists on purpose (that is the lesson).

Preview features (only the two read quests mention them; no graded quest enables preview):
`--enable-preview` would go in `tasks.withType<JavaCompile> { options.compilerArgs.add("--enable-preview") }` and
`tasks.test { jvmArgs("--enable-preview") }`. Not used. See R2.

### Gradle run, daemon, timing

- Daemon reuse needs the same JDK and Gradle version, so the `java` (JDK 21) and `java-25` tracks use **separate daemons**.
  Switching track costs one cold daemon start (~3-5 s). Within `java-25` every starter pins exactly Gradle 9.8.1.
- Same `GRADLE_USER_HOME` as `java` (the Gradle 9.8.1 distribution is shared, ~130 MB saved). Cold download for `java-25` alone: JDK ~190 MB
  (once) + JUnit 6/AssertJ ~8 MB; world 03 adds the Boot 4 stack (~90-110 MB, Hibernate 7 + Tomcat 11 + Jackson 3; measure).
- Warm daemon, one file changed: **2-5 s** (same as `java`), config cache on. Compact-source and gatherer quests are plain `javac`: ~2-3 s.
- Boot 4 worlds, estimates (measure in the spike): standalone `RestTestClient.bindToController` <1.5 s, `@WebMvcTest` 2-4 s,
  `@DataJpaTest` 3-5 s (Hibernate 7), `@SpringBootTest(RANDOM_PORT)` 4-8 s. Boot 4's modular auto-config should not be slower than 3.5.
- The shared `gradle.properties` is the same as `java` (`caching`, `parallel`, `configuration-cache`).

### Cannot be auto-graded (or only partly)

| item | how it is graded instead |
|---|---|
| "Did you use `_`" / "did you use a gatherer" / "did you write `///`" | Source scan (regex over `src/main/java`) as a **secondary** objective, behaviour objectives carry the rest. Says so in the lesson. A learner can cheat; nothing is lost. |
| Compact source file (01-03) | Implicit class is `final`, in the **unnamed package**, with a compiler-chosen name. Tests load it by file name via reflection and call the instance `main`; output captured from `System.out`. Starter for this quest lives in the unnamed package (breaks the `oxide.<slug>` convention; content-check must allow it). |
| Markdown doc comments | Test runs the JDK doc tool in-process (`ToolProvider.getSystemDocumentationTool()`) on the learner's file into a `@TempDir` and checks the generated HTML (`<table>`, `<code>`). ~2 s. |
| JSpecify null-safety | The *checker* (NullAway / IntelliJ) is not run. Graded by reflection: `@NullMarked` on the package, type-use `@Nullable` present on a return type (both runtime-retained), plus a source scan for leftover `org.springframework.lang.Nullable`. |
| Gradle script edits (03-02, 03-03) | As in `java` 06-09: graded by effect (version reported at runtime, slice tests only load with the right starters) plus a text check on `build.gradle.kts`. |
| Structured concurrency, primitive patterns, stable values | Preview in 25: **read quests only**. |
| JVM flags (compact headers, Shenandoah) | Read only; no measurable test. |
| "Boot really runs" | Only 04-06 binds a port, same fallback as `java` R4. |

---

## 2. Worlds

Legend: **ex** exercise, **win** quick win, **read** quiz (15 XP), **boss**. "requires (java)" = the `java` quest that this quest
upgrades or builds on. **World-level `requires`** (the first quest of the world, `requires` in its `quest.toml`; later quests default to the previous one):

| world | first quest requires |
|---|---|
| 00 Setup | `java/00-setup/08-boss-tip-calculator` |
| 01 Language | `java/03-collections/11-boss-log-analyzer` (records, sealed, patterns and streams are in 02-03; NeoForge-latest learners have only 00-03 of Java) |
| 02 Library and runtime | `java/07-concurrency/11-boss-link-checker` |
| 03 Spring Boot 4 | `java/08-spring-boot/12-boss-bookshelf-api` |
| 04 Capstone | `java/09-capstone/08-boss-ship-it` |

Quests that need something from a later `java` world than their predecessor name it in the last column; set that as an extra `requires`.
All quest ids below are `java-25/<world>/<NN-slug>`; `java` ids are `java/<world>/<NN-slug>` from `java.md`.

### World 00 - Setup (`00-setup`)
Spine: nothing built yet; JDK 25 works, one green tick, the map of what changed. First quest under 5 min. No boss.

| # | slug | idea | objectives | min | XP | kind | requires (java) |
|---|---|---|---|---|---|---|---|
| 01 | install-temurin-25 | Oxide detects JDK 25, shows the ~190 MB download, offers `winget install`; why 21 and 25 live side by side | (quiz) | 3 | 15 | read | `java/00-setup/01-whats-a-jdk` |
| 02 | toolchain-check | starter has the `toolchain` block; make `serverBannerReportsJavaVersion()` return `Runtime.version().feature()`; first green tick on 25 | `runsOnJava25` | 4 | 30 | win | `java/00-setup/02-hello-oxide` |
| 03 | junit-6 | starter still on JUnit BOM 5.14.4: bump to 6.0.3 and keep the tests green; what 6 dropped (JUnit 4 vintage), what stays | `junitPlatformIsVersion6`, `statusLineStillFormats` | 8 | 40 | ex | `java/06-build-test/01-junit-anatomy` |
| 04 | whats-new-map | the 21 to 25 map per world, final vs preview, what this track teaches and skips | (quiz) | 4 | 15 | read | `java/00-setup/05-project-layout` |

### World 01 - Language 22-25 (`01-language`)
Project: **Server status bot**. A bot that pings Minecraft servers and prints a status line. Each quest adds a feature the new syntax makes shorter or safer.

| # | slug | idea | objectives | min | XP | kind | requires (java) |
|---|---|---|---|---|---|---|---|
| 01 | constructor-bodies | `ServerEntry extends Entry`: parse `"host:port"` and reject bad input **before** `super(...)` (flexible constructor bodies, final in 25, JEP 513), no more static helper trick | `rejectsBlankHostBeforeSuper`, `parsesPortBeforeSuper`, `superNotRunOnBadInput` | 10 | 55 | ex | `java/02-oop/06-inheritance-and-super` |
| 02 | module-imports | `import module java.base;` and `import module java.net.http;` replace twelve imports in the pinger; `java.sql` + `java.base` makes `Date` ambiguous: fix with one single-type import (final in 25, JEP 511) | `pingerCompilesWithModuleImports`, `ambiguousDateResolvedBySingleImport` | 6 | 40 | win | `java/04-exceptions-io/08-java-time` |
| 03 | compact-main | the whole bot as one file: no `class`, no `public static`, instance `void main()`, `IO.println` (compact source files + instance main, final in 25, JEP 512) | `mainIsInstanceMethodWithoutArgs`, `printsStatusLine`, `sourceDeclaresNoClass` | 8 | 50 | ex | `java/00-setup/06-main-and-args` |
| 04 | unnamed-variables | `_` in `catch (NumberFormatException _)`, `case Online(var players, _)`, lambda params, `for (var _ : list)` (final in 22, JEP 456): summarize pings without naming what you ignore | `badPortIgnoresException`, `playersCountedWithoutLatency`, `usesUnderscoreForUnusedBindings` | 8 | 45 | win | `java/02-oop/11-sealed-and-pattern-switch` |
| 05 | primitive-patterns | **read**: `case int i`, `instanceof` on primitives, exact conversions: preview in 25 (3rd preview, JEP 507; 4th in 26), so what the bot's `switch` over a `long` would look like later and why we do not teach it yet | (quiz) | 3 | 15 | read | `java/02-oop/11-sealed-and-pattern-switch` |
| 06 | stream-gatherers | ping history: `windowSliding(3)` to spot flapping, `windowFixed` for hourly batches, `scan` for running uptime, a custom `dedupeConsecutive()` gatherer (final in 24, JEP 485) | `slidingWindowFlagsFlapping`, `fixedWindowBatchesHistory`, `customGathererDropsRepeats` | 12 | 70 | ex | `java/03-collections/10-flatmap-reduce` |
| 07 | boss-status-bot | one compact `StatusBot.java` pings a list of servers through an injected fake `Pinger`, validates addresses in a constructor body, prints per-server lines, flags flapping, rolls history into hourly summaries | `rejectsBadAddressBeforeSuper`, `reportsOnlineWithPlayers`, `reportsOfflineReason`, `detectsFlappingWithSlidingWindow`, `batchesHistoryIntoHourlySummaries`, `compactMainPrintsReport` | 40 | 350 | boss | `java/03-collections/11-boss-log-analyzer` |

World 01 totals: 7 quests, 87 min, 625 XP.

### World 02 - Library and runtime (`02-runtime`)
Project: **Modpack updater**. Reads mod jars, runs update checks in parallel, reports which mods need which Java.

| # | slug | idea | objectives | min | XP | kind | requires (java) |
|---|---|---|---|---|---|---|---|
| 01 | scoped-values | pass the current "update run" (id, pack name) to every task without a `ThreadLocal` or an extra parameter: `ScopedValue.where(RUN, run).call(...)`, rebinding in a nested scope, unbound outside (final in 25, JEP 506) | `valueVisibleInsideScope`, `unboundOutsideScope`, `innerBindingShadowsOuter` | 10 | 60 | ex | `java/07-concurrency/09-virtual-threads` |
| 02 | virtual-threads-no-pinning | 50 downloads, each in a `synchronized` block that sleeps, on **one carrier thread** (`-Djdk.virtualThreadScheduler.parallelism=1`): in 21 they pin and run one by one, in 25 they overlap (JEP 491, final in 24) | `synchronizedTasksOverlapOnOneCarrier`, `fiftyDownloadsFinishUnderOneSecond` | 8 | 50 | ex | `java/07-concurrency/09-virtual-threads` |
| 03 | class-file-api | mod jar scanner: parse `.class` bytes with `java.lang.classfile` (final in 24, JEP 484): read the major version (65 = Java 21, 69 = Java 25), list methods, find methods with a given annotation | `readsMajorVersion`, `listsMethodNames`, `findsAnnotatedMethods` | 12 | 70 | ex | `java/04-exceptions-io/05-paths-and-files` |
| 04 | markdown-doc-comments | document the updater API with `///` Markdown comments: lists, a table, `` `code` `` and `[links]` instead of `<p>`/`{@code}` (JEP 467, final in 23) | `javadocRendersTable`, `javadocRendersCodeSpan` | 8 | 45 | win | `java/02-oop/03-encapsulation` |
| 05 | preview-and-runtime-flags | **read**: structured concurrency is still preview (JEP 505, 5th preview in 25; 6th in 26): what the API looks like and why we skip it; plus one-liners: compact object headers (`-XX:+UseCompactObjectHeaders`, final in 25), generational Shenandoah (`-XX:+UseShenandoahGC -XX:ShenandoahGCMode=generational`, final in 25), non-generational ZGC removed in 24, AOT cache ergonomics; what that means for a modpack server | (quiz, 3 questions) | 4 | 15 | read | `java/07-concurrency/08-completablefuture-combine` |
| 06 | boss-parallel-updater | check every mod jar in a folder (in-memory fakes) on virtual threads, each task reads the run context from a scoped value, class-file scan finds mods built for a newer Java than the server, results in input order | `scansAllJarsForJavaVersion`, `flagsModsNeedingNewerJava`, `runsChecksOnVirtualThreads`, `runContextVisibleInEveryTask`, `noPinningUnderOneCarrier` | 50 | 400 | boss | `java/07-concurrency/11-boss-link-checker` |

World 02 totals: 6 quests, 92 min, 640 XP.

### World 03 - Spring Boot 4 (`03-boot-4`)
Project: **Modpack registry API** (the `java` Bookshelf service in its games flavour; plain flavour = bookshelf). The starter is the finished Boot 3.5.16
service from `java/08-spring-boot/12`; each quest migrates one layer. Tests are the spec and already use Boot 4 / Framework 7 APIs
where the quest is about them.

| # | slug | idea | objectives | min | XP | kind | requires (java) |
|---|---|---|---|---|---|---|---|
| 01 | whats-in-boot-4 | **read**: Spring Framework 7, Jakarta EE 11 (Servlet 6.1, Persistence 3.2, Tomcat 11, Hibernate 7), modular starters, Jackson 3, JSpecify, API versioning, JUnit 6; what you do first: reach the latest 3.5.x, fix deprecations | (quiz) | 4 | 15 | read | `java/08-spring-boot/01-what-spring-does` |
| 02 | bump-boot-plugin | change the Gradle plugin 3.5.16 to 4.1.1; build breaks on `spring-boot-starter-web`: use the temporary `spring-boot-starter-classic` / `-test-classic` bridge to get green | `bootVersionIs4`, `appContextStartsOnBoot4` | 10 | 55 | ex | `java/08-spring-boot/04-rest-controller` |
| 03 | modular-starters | drop the classic bridge: `starter-web` to `starter-webmvc`, add `starter-validation`, `starter-data-jpa`, and per-technology test starters `starter-webmvc-test`, `starter-data-jpa-test`; slice annotations now live in new packages (`org.springframework.boot.webmvc.test.autoconfigure.WebMvcTest`) | `webMvcSliceTestRuns`, `dataJpaSliceTestRuns`, `classicBridgeRemoved` | 10 | 50 | ex | `java/08-spring-boot/09-service-layer-webmvctest` |
| 04 | jackson-3-imports | `com.fasterxml.jackson.databind.ObjectMapper` to `tools.jackson.databind.json.JsonMapper`; annotations stay in `com.fasterxml.jackson.annotation`; `JacksonException` is unchecked; `@JsonComponent` to `@JacksonComponent` | `roundTripsModEntryWithJsonMapper`, `badJsonThrowsJacksonException`, `customSerializerUsesJacksonComponent` | 10 | 60 | ex | `java/08-spring-boot/06-request-body-and-status` |
| 05 | jackson-3-config | property renames `spring.jackson.read.*` / `write.*` to `spring.jackson.json.read.*` / `json.write.*`; Jackson 3 defaults vs 2 (`use-jackson2-defaults`); `JsonMapperBuilderCustomizer` replaces `Jackson2ObjectMapperBuilderCustomizer` | `writeFeaturesAppliedUnderJsonNamespace`, `customizerAppliesNamingStrategy` | 8 | 45 | win | `java/08-spring-boot/03-config-and-properties` |
| 06 | rest-test-client | `RestTestClient` (Framework 7): `bindToController(...)` standalone, bound to `MockMvc`, assertions with `expectStatus()`/`jsonPath`; `@SpringBootTest` no longer auto-configures MockMvc: add `@AutoConfigureMockMvc` / `@AutoConfigureRestTestClient` | `getsModEntryAsJson`, `postReturns201WithLocation`, `unknownModIs404` | 12 | 65 | ex | `java/08-spring-boot/06-request-body-and-status` |
| 07 | jspecify-null-safety | `@NullMarked` in `package-info.java`, `org.jspecify.annotations.Nullable` replaces `org.springframework.lang.Nullable`; make a repository lookup honest about null | `packageIsNullMarked`, `findByLoaderReturnTypeIsNullable`, `springLangNullableGone` | 10 | 55 | ex | `java/05-generics-optional/06-optional-basics` |
| 08 | api-versioning | `/mods` serves a v1 and a v2 shape: `spring.mvc.apiversion.use.header=X-API-Version` and `@GetMapping(version = "1.0")` / `"2.0+"`; missing or unknown version is 400 | `v1ReturnsLegacyShape`, `v2ReturnsNewShape`, `missingVersionIs400`, `unsupportedVersionIs400` | 12 | 70 | ex | `java/08-spring-boot/04-rest-controller` |
| 09 | http-service-client | call an upstream mod API through an `@HttpExchange` interface (`HttpServiceProxyFactory` / `@ImportHttpServices`), tested with `MockRestServiceServer` | `getCallsUpstreamPath`, `postSendsJsonBody` | 10 | 60 | ex | `java/08-spring-boot/06-request-body-and-status` |
| 10 | boss-registry-on-boot-4 | the whole registry service (CRUD, validation, errors, JPA on H2) migrated to Boot 4.1.1 with Boot-4 style tests; same behaviour as the `java` boss | `createsMod`, `rejectsInvalidMod`, `getsModById`, `unknownIdIs404`, `listsPagedByLoader`, `deleteIs204`, `repoPersistsAcrossLayers`, `bootVersionIs4` | 70 | 400 | boss | `java/08-spring-boot/12-boss-bookshelf-api` |

World 03 totals: 10 quests, 156 min, 875 XP.

Starter notes: quest 02 starts on 3.5.16 (compiles, `bootVersionIs4` red). From 03 on every starter's build already says 4.1.1; 03 starts on the classic bridge.
04, 05, 07, 09 ship the service with only that layer left on the old API but compiling (old import paths keep working through `spring-boot-jackson2` /
deprecated annotations where Boot 4 still has them; otherwise the quest sets `starter_fails_to_compile = true` and the lesson says so).

### World 04 - Capstone: habits on 25 (`04-capstone`)
Project: the **habits** REST service + CLI from `java/09-capstone`, migrated. Starter = the `java` capstone solution (Boot 3.5.16, JDK 21, JUnit 5) plus the same test classes; objectives reuse
the `java` test names so "the same tests green" is literal. Everything is tested without a server except the boss.

| # | slug | idea | objectives | min | XP | kind | requires (java) |
|---|---|---|---|---|---|---|---|
| 01 | migration-plan | **read**: order of attack (toolchain, domain, service, CLI, boss), what to leave alone, how to know a step is done | (quiz) | 4 | 15 | read | `java/09-capstone/01-design` |
| 02 | habits-on-jdk-25 | move the whole project to the toolchain block, JUnit 6.0.3 and `junit-platform-launcher`; the existing domain tests must stay green | `streakCountsConsecutiveDays`, `streakResetsAfterGap`, `todayCheckInIsIdempotent` | 8 | 45 | win | `java/09-capstone/02-domain-model` |
| 03 | modernize-domain | use 25 where it pays: `windowSliding` gatherer for the longest streak, `_` for unused bindings, validation before `super(...)` in a `Habit` subtype; old behaviour unchanged | `longestStreakViaGatherer`, `streakStillResetsAfterGap`, `unusedBindingsAreUnnamed` | 12 | 65 | ex | `java/09-capstone/02-domain-model` |
| 04 | habits-service-on-boot-4 | service, controller and JPA repo to Boot 4.1.1: modular starters, Jackson 3, JSpecify package, `RestTestClient`; the `java` test names must pass | `listsHabits`, `postsCheckIn`, `unknownHabitIs404`, `jpaRepoSatisfiesSameContract` | 15 | 75 | ex | `java/09-capstone/05-jpa-persistence` |
| 05 | cli-on-25 | the CLI as an ordinary class with an **instance** `main`, sealed `Command` records with `_` patterns; same parser tests | `parsesAdd`, `parsesDone`, `unknownCommandReportsUsage`, `mainIsInstanceMethod` | 10 | 55 | ex | `java/09-capstone/06-cli-parser` |
| 06 | boss-ship-it-on-25 | `@SpringBootTest(RANDOM_PORT)` boots the service, the real CLI client runs against it, `bootJar` builds on 25; plus a new v2 endpoint (`X-API-Version: 2.0` returns `longestStreak`); then run it yourself (manual victory lap) | `serviceStartsOnRandomPort`, `cliAddsAndListsAgainstRealServer`, `streakShownAfterTwoCheckIns`, `bootJarTaskProducesJar`, `apiV2ReturnsLongestStreak`, `allJavaTrackTestsStillGreen` | 90 | 500 | boss | `java/09-capstone/08-boss-ship-it` |

World 04 totals: 6 quests, 139 min, 755 XP. `allJavaTrackTestsStillGreen` is graded by running the unchanged `java` capstone test classes (copied into the starter) in the same Gradle run.

---

## 3. "Changed since 21" cheat sheet

### Java 21 to 25

| feature | Java 21 way | Java 25 way | where it matters |
|---|---|---|---|
| Validate before `super(...)` | static helper called inside the `super(...)` argument | statements before `super(...)` (final 25, JEP 513) | status bot `ServerEntry` (01-01) |
| Imports | one `import` per class | `import module java.base;` (final 25, JEP 511); beware ambiguous names (`Date`) | status bot pinger (01-02) |
| Hello world | `public class X { public static void main(String[] a) }` | compact file, instance `void main()`, `IO.println` (final 25, JEP 512); implicit class is final, unnamed package, name not usable in source | status bot (01-03), CLI (04-05) |
| Ignored values | `catch (E ignored)`, `case P(var a, var b)` with unused `b` | `_` (final 22, JEP 456) | pattern matching in the bot (01-04) |
| Primitives in patterns | manual range checks, boxing | `case int i`, exact conversions: **preview** (JEP 507, 4th preview in 26) | read only (01-05) |
| Sliding / batch streams | manual loops or custom collectors | `stream.gather(Gatherers.windowSliding(n))`, `windowFixed`, `scan`, `fold`, `mapConcurrent`, custom `Gatherer` (final 24, JEP 485) | flapping detection (01-06), streaks (04-03) |
| Per-request context | `ThreadLocal` (mutable, leaks, copied to children) | `ScopedValue.where(K, v).call(...)` (final 25, JEP 506) | modpack updater run context (02-01) |
| Fan-out and cancel | `ExecutorService` + `Future`s | `StructuredTaskScope`: still **preview** in 25 (JEP 505) and 26 (JEP 525) | read only (02-05) |
| Virtual threads + `synchronized` | pinned the carrier thread | no pinning (final 24, JEP 491) | parallel updater (02-02, 02-06) |
| Reading bytecode | ASM / ByteBuddy | `java.lang.classfile` (final 24, JEP 484) | mod jar scanner (02-03) |
| API docs | HTML + `{@code}` in `/** */` | Markdown in `///` (final 23, JEP 467) | updater API docs (02-04) |
| JVM memory/GC | default 12-byte headers; generational ZGC optional | `-XX:+UseCompactObjectHeaders` (final 25, JEP 519), generational Shenandoah (final 25, JEP 521), non-generational ZGC removed (24) | read only, matters for modpack servers (02-05) |
| Test framework | JUnit 5.14.4 | JUnit 6.0.3 (Java 17+, same Jupiter annotations, no vintage engine) | every starter (00-03) |
| Build | toolchain block absent, `JAVA_HOME` = 21 | toolchain block, `JAVA_HOME` = 25, Gradle >= 9.1 (we use 9.8.1) | all starters |

Not covered: FFM API (final 22), multi-file source launch (22), PEM encodings (preview), Stable Values (preview in 25, renamed Lazy Constants in 26), Vector API (incubator),
AOT class loading (24-25, one line in 02-05), Remove 32-bit x86.

### Spring Boot 3.5.16 to 4.1.1

| feature | Boot 3.5 way | Boot 4 way | where it matters |
|---|---|---|---|
| Baseline | Framework 6.2, Jakarta EE 10, Tomcat 10.1, Hibernate 6.6 | Framework 7.0, Jakarta EE 11 (Servlet 6.1, Persistence 3.2), Tomcat 11, Hibernate 7.4, Validator 9 | whole world 03; mostly invisible |
| Web starter | `spring-boot-starter-web` | `spring-boot-starter-webmvc` (bridge: `spring-boot-starter-classic`) | 03-02, 03-03 |
| Test starters | one `spring-boot-starter-test` | plus `spring-boot-starter-<tech>-test` per technology (`webmvc-test`, `data-jpa-test`, `security-test`) | 03-03 |
| Slice annotation imports | `org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest` | `org.springframework.boot.webmvc.test.autoconfigure.WebMvcTest` (pattern: `org.springframework.boot.<tech>.test.autoconfigure`) | 03-03 |
| Jackson | 2.x, `com.fasterxml.jackson.databind.ObjectMapper`, checked `JsonProcessingException` | 3.x, `tools.jackson.databind.json.JsonMapper` (immutable, built by builder), unchecked `JacksonException`; annotations stay in `com.fasterxml.jackson.annotation` | 03-04, 03-05, capstone |
| Jackson config | `spring.jackson.read.*`, `write.*`, `Jackson2ObjectMapperBuilderCustomizer`, `@JsonComponent` | `spring.jackson.json.read.*` / `json.write.*`, `JsonMapperBuilderCustomizer`, `@JacksonComponent`; compat: `spring-boot-jackson2`, `spring.jackson.use-jackson2-defaults` | 03-05 |
| MockMvc in `@SpringBootTest` | auto-configured | `@AutoConfigureMockMvc` required | 03-06 |
| HTTP test client | `TestRestTemplate` / `WebTestClient` auto-configured | `RestTestClient` (`@AutoConfigureRestTestClient`, `org.springframework.test.web.servlet.client`); `TestRestTemplate` needs `spring-boot-resttestclient` + `@AutoConfigureTestRestTemplate` | 03-06, 04-06 |
| Mock beans | `@MockitoBean` (`@MockBean` deprecated) | `@MockBean` / `@SpyBean` removed; `@MockitoBean` fields only, not in `@Configuration` | already in `java` 08-09 |
| Null annotations | `org.springframework.lang.Nullable` / `@NonNull` | JSpecify `@NullMarked` / `@Nullable` (type-use) | 03-07 |
| Versioned APIs | `/v1/...` paths or custom headers by hand | `@RequestMapping(version = ...)`, `spring.mvc.apiversion.*`, `ApiVersionResolver` / `Parser` / `DeprecationHandler` | 03-08, 04-06 |
| Upstream HTTP calls | `RestTemplate` / hand-written `RestClient` | `@HttpExchange` interfaces, auto-configured via `@ImportHttpServices` and `spring.http.serviceclient.*` | 03-09 |
| Properties | `spring.dao.exceptiontranslation.enabled`, `spring.data.mongodb.host` ... | `spring.persistence.exceptiontranslation.enabled`, `spring.mongodb.*`; run `spring-boot-properties-migrator` temporarily | note in 03-02 |
| Moved packages | `org.springframework.boot.autoconfigure.domain.EntityScan` | `org.springframework.boot.persistence.autoconfigure.EntityScan`; `hibernate-jpamodelgen` to `hibernate-processor` | 03-10 (only if the bookshelf uses them) |
| Defaults changed | probes off, LiveReload on | liveness/readiness probes on, DevTools LiveReload off | footnote only |
| Removed | Undertow, Spock, classic uber-jar loader, Spring Session Hazelcast/Mongo | gone | not used in our projects |

---

## 4. Risks and open questions

**R1. Boot 4 ecosystem lag.** The `java` track needs: H2 (2.4.240 is in the BOM, fine), Mockito (5.23.0, fine), AssertJ (3.27.7, fine), Hibernate 7.4.5
(JPA 3.2: `@Entity` basics unchanged, but HQL strictness and `hibernate-processor` rename can bite derived queries), Hibernate Validator 9.1.3. Anything
extra (Flyway, springdoc-openapi, Testcontainers) is not used. Rule: no dependency outside the Boot BOM without checking Boot-4 support first.

**R2. Preview flags and engine.** No graded quest needs `--enable-preview`; the two read quests show code in the lesson, not in a starter. Reason: preview class files only
load on the exact JDK major, preview APIs change every release (structured concurrency changed shape in 25: `StructuredTaskScope.open()` + `Joiner`), and JDK 26 is already out with
SC in its 6th preview and primitive patterns in 4th. If a future quest needs a preview, the engine must pass `--enable-preview` in both `compileJava` and `test`.

**R3. Gradle/JDK 25 support.** Gradle 9.1.0+ runs on 25; 9.8.1 is fine. Risks: plugins that read class files (Kotlin DSL, JaCoCo, ByteBuddy used by Mockito) must know class major 69.
Mockito 5.23.0 and Boot-managed ByteBuddy handle 25; verify in the measurement spike with a `-javaagent` quest. `io.spring.dependency-management` 1.1.7 predates Boot 4:
if it misbehaves, fall back to `implementation(platform(SpringBootPlugin.BOM_COORDINATES))` (Q3).

**R4. Compact source files vs the layout rules.** Unnamed package, compiler-chosen class name, `final`: the test must reflect it by file name, and content-check must tolerate a starter outside `oxide.<slug>`.
If reflection proves brittle on Windows paths or the Gradle test classpath, grade 01-03 through `ProcessBuilder` running `java StatusBot.java` instead (slower, ~1 s per call).

**R5. Doc tool in tests.** 04-02 runs `javadoc` in-process; first call loads `jdk.javadoc` (~1.5 s). If it is flaky, replace by a source scan for `///` and a rendered-HTML check made once at content-build time.

**R6. Pinning test is timing-based.** 02-02 relies on `jdk.virtualThreadScheduler.parallelism=1` (set in `tasks.test` `jvmArgs`) and a generous bound (<1 s vs ~5 s pinned).
On a loaded machine keep a 3x margin; the 21 behaviour is documented in the lesson, never run.

**R7. JUnit 6 vs the `java` track's JUnit 5.** Learners see 5 in `java` and 6 here. Annotations and test names are identical; engine parsing unchanged. Boot 4 forces 6.0.3 anyway, so
splitting by track is cleaner than backporting. If the `java` track later moves to 6, delete 00-03.

**R8. Migration starters must compile.** Boot 3 to 4 changes imports, so a "still on 3.5 API" starter next to Boot-4 tests does not compile. Mitigation in section 2: per-quest dependency
lists keep old paths alive (`spring-boot-starter-classic`, `spring-boot-jackson2`), otherwise `starter_fails_to_compile = true` with a lesson note. Cost: 03-04, 03-06, 03-07 and 04-04 need careful per-quest starters; budget extra time there.

**R9. Download size.** JDK 25 ~190 MB, Boot 4 stack ~90-110 MB, all on top of the `java` track's Gradle distribution. Shown before download in 00-01 and 03-01.

**R10. XP and rule fit.** Capstone boss is 500 XP (top of the 300-500 range, same as `java` 09-08). World 00 has no boss (as allowed for Setup worlds in the Fabric plan).

Open questions:

- **Q1.** Should `java-25` replace `java` content for learners who start fresh (skip 21 entirely)? Plan assumes no: it is a delta and `requires` the `java` bosses.
- **Q2.** Games flavour for world 03/04: "Modpack registry" (mods and loaders) is assumed for the Bookshelf service; confirm or keep bookshelf as the plain flavour only.
- **Q3.** Measurement spike before content: Boot plugin 4.1.1 with `io.spring.dependency-management` 1.1.7 vs `platform(BOM)`, Mockito agent on JDK 25, and real cold/warm timings for the Boot 4 slices.
- **Q4.** Ship order: worlds 00-02 (no Spring, ~200 min) first, then 03-04 once the `java` world 08-09 content exists (03 literally starts from its boss).
- **Q5.** Mutation-grading is still deferred (see `java` Q1); nothing in this track needs it.
