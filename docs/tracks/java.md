# Track plan: Java

Status: plan (2026-10-19). Runner `gradle`, JUnit 5, JDK 21. Design context in `docs/TRACKS.md`,
quest format in `docs/CONTENT.md`. No starters or lessons exist yet; this doc is the contract for
the content sessions.

**Totals:** 10 worlds, 103 quests, 8175 XP, ~22 h of content.

---

## 1. Toolchain and runner

### Pinned versions (checked on Maven Central / services.gradle.org, 2026-10-19)

| thing | pin | notes |
|---|---|---|
| JDK | 21 (Temurin 21.0.12 installed) | LTS. Records, sealed, switch patterns, virtual threads are final in 21. `StructuredTaskScope` and string templates are **preview** in 21: do not teach. |
| Gradle (wrapper) | **9.8.1** (current stable) | Gradle 9 runs on JDK 17+. Wrapper per starter. |
| JUnit | **5.14.4** (latest 5.x) | JUnit **6.1.3** exists (6.x needs Java 17+). We stay on 5: TRACKS.md says JUnit 5, Boot 3.5 manages 5.12, and most tutorials are 5. Moving to 6 later = change the BOM line only (R9). |
| AssertJ | **3.27.7** | 4.0.0 is still milestone. |
| Mockito | **5.24.0** standalone; Boot-managed **5.17.0** in Spring worlds | In Spring worlds do not pin; the Boot BOM decides. |
| Spring Boot | **3.5.16** (last OSS 3.x, 2026-06-25) | Out of OSS support; Boot 4.1.1 is current. See R1 / Q3. |
| Boot 3.5.16 BOM | H2 2.3.232, Hibernate 6.6.53, Spring Framework 6.2.19, Tomcat 10.1.55 | |

Worlds 00-07 use the pins above directly. Worlds 08-09 use the `org.springframework.boot` Gradle plugin
3.5.16 and take JUnit/AssertJ/Mockito from `spring-boot-starter-test`.

### Minimal starter `build.gradle.kts` (worlds 00-07)

```kotlin
plugins { java }

repositories { mavenCentral() }

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.14.4"))
    testImplementation("org.junit.jupiter:junit-jupiter")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")   // required on Gradle 9
    testImplementation("org.assertj:assertj-core:3.27.7")
}

tasks.test {
    useJUnitPlatform()
    testLogging { events("failed"); exceptionFormat = org.gradle.api.tasks.testing.logging.TestExceptionFormat.FULL }
}
```

- `junit-platform-launcher` on `testRuntimeOnly` is mandatory with Gradle 9 (missing = "Failed to load JUnit Platform").
- No `java { toolchain {...} }` block: it needs a resolver plugin to auto-download JDKs. Oxide sets `JAVA_HOME` to JDK 21 instead.
- Mockito quests add `org.mockito:mockito-junit-jupiter:5.24.0` and a `-javaagent` for mockito-core in `tasks.test`;
  without it JDK 21 prints a "self-attaching" warning (harmless, noisy; the engine also filters it, R6).

### Starter layout (per quest)

```
starter/
  gradlew, gradlew.bat, gradle/wrapper/{gradle-wrapper.jar, gradle-wrapper.properties}
  settings.gradle.kts           rootProject.name = "<unique slug>"
  build.gradle.kts              as above (+ quest-specific deps)
  src/main/java/oxide/<slug>/*.java       learner edits; compiles as shipped
  src/test/java/oxide/<slug>/*Test.java   the spec; objective = test method name
  (no build/, no .gradle/)
```

- Wrapper files are identical everywhere; the engine can inject one shared copy (Q2).
- Package `oxide.<slug_with_underscores>`; plain class names (`Greeter`, `Bank`).
- Starters **must compile**; fill-ins `throw new UnsupportedOperationException("TODO: ...")` or return a wrong value.
  Compile-error quests set `starter_fails_to_compile = true`, as in Rust.
- Test names are camelCase behaviours. One idea per test so one tick = one idea. AssertJ is the default assertion style from 00-02 on.
- Speed knobs in a shared `gradle.properties`: `org.gradle.caching=true`, `org.gradle.parallel=true`, `org.gradle.configuration-cache=true`.

### Objectives from JUnit XML

Parse `build/test-results/test/TEST-*.xml`. JUnit 5 writes `<testcase name="dividesWithRemainder()" classname="oxide.x.MathTest">`:
**strip the trailing `()`** and suffix-match the method name. Parameterized tests get names like `fizzbuzz(int, String)[3]`:
an objective is green when **all** cases with that method name pass. `@Nested` works (classname `Outer$Inner`, same method matching).
`<failure message=...>` holds the AssertJ text.

### How fast

- **Cold** (no Gradle dist, empty cache): Gradle ~130 MB + JUnit/AssertJ ~8 MB, first `test` ~40-70 s. Once.
- **Warm daemon, one file changed:** 2-5 s (TRACKS.md figure); config cache should push toward 1.5-3 s.
- **Different quest, warm daemon:** ~4-8 s (dependency cache warm, project configured fresh). Daemon reuse across
  projects needs the same JDK and Gradle version, so **every starter pins exactly 9.8.1**.
- This machine already has `~/.gradle` (3.7 GB, wrappers 8.10 to 9.4.1). Oxide should use its **own** short
  `GRADLE_USER_HOME` (R8) so a reset is safe and nothing leaks into the user's other Gradle work.
- Spring worlds: first resolve ~70-90 MB; then standalone MockMvc <1.5 s, `@WebMvcTest` ~2-4 s, `@DataJpaTest` ~3-5 s,
  `@SpringBootTest` ~4-8 s (estimates, measure when building 08).

### What the learner sees

| situation | shown |
|---|---|
| compile error | `src/main/java/oxide/x/Foo.java:12: error: incompatible types: String cannot be converted to int` parsed to file:line + message; objectives greyed ("fix the compile error first"). Engine drops Gradle's `FAILURE: Build failed` boilerplate after the first `error:` group. |
| failing assertion | AssertJ block (`expected: 4` / `but was: 5`) plus the test's file:line, under the failing objective. |
| `UnsupportedOperationException: TODO: ...` | shown as "not written yet: <message>", not as a stack trace. Engine special-cases the `TODO:` prefix. |
| infinite loop / deadlock | `junit.jupiter.execution.timeout.default=5s` in `junit-platform.properties`; shows "timed out after 5 s". Gradle-level kill at 60 s. |
| stdin / `System.exit` | tests never read stdin; code takes a `Reader`/`Scanner` parameter. |

### Cannot be auto-graded (or only partly)

- **"Write a test" quests.** Learner-authored tests have no fixed names. Plan: starter ships a correct implementation plus
  `mutants/` (3 broken variants, outside the source set). The engine runs the learner's tests against real + each mutant;
  objectives are synthetic (`passesOnCorrectCode`, `catchesOffByOne`, ...). Needs new runner support (Q1). Used in 06-03, 06-05, 06-11.
- **Design quality, naming, style.** Behaviour only.
- **Real network / server / clock / files.** Faked or loopback: JDK `HttpServer` on port 0, `@TempDir`, injected `Clock`.
- **"Does it really start and listen?"** Only the capstone boss does; the final "run it yourself" is manual.
- **Probabilistic concurrency bugs.** Race quests check structure (reflection: `AtomicInteger`, `synchronized`) plus a stress test
  (8 threads x 50,000 increments) that fails on the broken starter with near-certainty. Lessons say "passes = no race seen", not proof.
- **Gradle script edits** (06-09): graded indirectly (code needs the added dependency/task); other buildscript errors show as raw Gradle output.
- **IDE skills** (debugger, refactoring): read-only advice.

---

## 2. Worlds

Legend: **ex** exercise, **win** quick win (lower XP/minutes), **read** read-kind (quiz, 15 XP), **boss**.
Objectives are JUnit method names (`[[objectives]] test =`). Slugs are folder names without the `NN-`. Minutes are beginner estimates.

Cross-track: Minecraft worlds `requires` `java/02-oop` (and use lambdas from 03). Minecraft needs only worlds 00-03.

| world | quests | read | win | boss | minutes | XP |
|---|---|---|---|---|---|---|
| 00 Setup | 8 | 3 | 1 | 1 | 71 | 510 |
| 01 Basics | 11 | 1 | 2 | 1 | 121 | 775 |
| 02 OOP | 12 | 1 | 2 | 1 | 154 | 935 |
| 03 Collections & Streams | 11 | 1 | 2 | 1 | 127 | 795 |
| 04 Exceptions & IO | 10 | 1 | 2 | 1 | 124 | 750 |
| 05 Generics & Optional | 9 | 1 | 2 | 1 | 110 | 695 |
| 06 Build & Test | 11 | 2 | 2 | 1 | 148 | 860 |
| 07 Concurrency | 11 | 1 | 2 | 1 | 144 | 890 |
| 08 Spring Boot | 12 | 1 | 1 | 1 | 186 | 1050 |
| 09 Capstone | 8 | 1 | 1 | 1 | 170 | 915 |
| **total** | **103** | 13 | 17 | 10 | **1355** (~22 h) | **8175** |


### World 00 - Setup (`00-setup`)
Toolchain works, first green tick, learner can read test output. First quest under 5 min.

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | whats-a-jdk | JDK vs JRE, javac, java, Gradle wrapper; Oxide checks JDK 21 and shows the ~140 MB download | (quiz) | 3 | 15 | read |
| 02 | hello-oxide | `greeting()` returns "Hello, Oxide!": first green tick | `greetingSaysHello` | 4 | 30 | win |
| 03 | reading-red | how to read an AssertJ failure (expected / but was, file:line) | (quiz) | 3 | 15 | read |
| 04 | compile-error | fix a type mismatch and a missing semicolon (compile-fail starter) | `compilesAndAddsTwoNumbers` | 6 | 40 | ex |
| 05 | project-layout | `src/main`, `src/test`, `build.gradle.kts`, where classes come from | (quiz) | 4 | 15 | read |
| 06 | main-and-args | `banner(String[] args)`: first arg or default name | `bannerUsesFirstArg`, `bannerDefaultsWithoutArgs` | 8 | 50 | ex |
| 07 | printing | `System.out.printf`, `String.format`, output captured by the test | `printsGreetingLine`, `formatsPriceWithTwoDecimals` | 8 | 45 | ex |
| 08 | boss-tip-calculator | parse "42.50", apply tip %, split n ways, format the receipt | `parsesAmount`, `appliesTipPercent`, `splitsEvenly`, `receiptIsFormatted` | 35 | 300 | boss |

### World 01 - Basics (`01-basics`)

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | primitives | `int` vs `long` vs `double`, overflow wraps silently | `intOverflowWraps`, `longAvoidsOverflow` | 6 | 40 | ex |
| 02 | integer-division | `/` and `%` on ints, casting to get a fraction | `dividesWithRemainder` | 4 | 30 | win |
| 03 | how-java-runs | source, bytecode, JVM, JIT, GC in one screen | (quiz) | 4 | 15 | read |
| 04 | if-and-ternary | `if`/`else`, `?:`, `&&` short-circuit: `clamp`, `signOf` | `clampKeepsInRange`, `signOfReturnsMinusOneZeroOne` | 8 | 45 | ex |
| 05 | switch-expressions | arrow `switch` returning a value: `daysInMonth`, `gradeFor` | `daysInMonthHandlesLeapYear`, `gradeForMapsBands` | 8 | 50 | ex |
| 06 | loops | `for`, `while`, enhanced for: `sumTo`, `countVowels` | `sumToAddsInclusive`, `countVowelsIgnoresCase` | 10 | 50 | ex |
| 07 | methods-and-overloads | overloads and varargs: `area(...)`, `sum(int...)` | `overloadPicksByType`, `varargsSumsAny` | 8 | 45 | ex |
| 08 | strings | immutability, `substring`, `equals` vs `==`: `reverse`, `isPalindrome` | `reversesText`, `palindromeIgnoresCaseAndSpaces`, `equalsComparesContent` | 10 | 55 | ex |
| 09 | text-blocks-and-format | text blocks, `formatted`, `StringBuilder` | `buildsTableWithStringBuilder`, `textBlockKeepsIndent` | 6 | 35 | win |
| 10 | arrays | index, `Arrays.sort`, 2D grid: `max`, `reverseInPlace`, `transpose` | `maxFindsLargest`, `reverseInPlaceMutates`, `transposeSwapsRowsAndCols` | 12 | 60 | ex |
| 11 | boss-wordle-scorer | score a guess against a target: green/yellow/gray with duplicate letters | `allGreenOnExactMatch`, `yellowForWrongPosition`, `duplicateLettersScoredOnce`, `rendersEmojiRow` | 45 | 350 | boss |

### World 02 - OOP (`02-oop`)

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | classes-and-fields | a `Counter` class: fields, instance methods | `startsAtZero`, `incrementAddsOne` | 6 | 40 | ex |
| 02 | constructors-and-this | overloaded constructors, `this(...)` chaining | `defaultConstructorSetsDefaults`, `chainedConstructorReusesFirst` | 6 | 35 | win |
| 03 | encapsulation | private fields, getters, how mutable state leaks | (quiz) | 4 | 15 | read |
| 04 | static-members | `static` counter and factory `of(...)`, when not to | `staticFactoryBuildsInstance`, `idsAreUniquePerInstance` | 8 | 45 | ex |
| 05 | interfaces | `Shape` interface, two impls, a default method | `circleAreaMatchesFormula`, `rectanglePerimeter`, `defaultDescribeUsesName` | 10 | 55 | ex |
| 06 | inheritance-and-super | `extends`, `super`, overriding: `Animal` to `Dog` | `dogOverridesSound`, `superCallKeepsBaseBehaviour` | 10 | 55 | ex |
| 07 | polymorphism | list of `Employee`, abstract `pay()`, no `instanceof` | `abstractMethodImplementedBySubtypes`, `payrollSumsAllSubtypes` | 10 | 60 | ex |
| 08 | equals-hashcode-tostring | the contract; value equality; HashSet misbehaves without it | `equalsComparesFields`, `hashCodeMatchesEquals`, `toStringIsReadable` | 12 | 65 | ex |
| 09 | enums | enum with fields and methods, `values()`, `switch` over enum | `enumCarriesFields`, `valueOfParsesName`, `nextWrapsAround` | 8 | 40 | win |
| 10 | records | immutable data: compact constructor validates, derived methods | `recordEqualsByValue`, `compactConstructorRejectsNegative`, `withMethodReturnsCopy` | 8 | 55 | ex |
| 11 | sealed-and-pattern-switch | `sealed interface Shape` + records + pattern `switch`, no `default` | `exhaustiveSwitchComputesArea`, `guardedPatternHandlesZero`, `recordPatternDeconstructs` | 12 | 70 | ex |
| 12 | boss-bank-ledger | accounts, sealed `Transaction` records (Deposit/Withdraw/Transfer), overdraft rule, statement string | `depositIncreasesBalance`, `overdraftRejected`, `transferMovesMoneyBothWays`, `statementListsInOrder`, `sealedSwitchIsExhaustive` | 60 | 400 | boss |

### World 03 - Collections & Streams (`03-collections`)

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | lists | `ArrayList`: add/remove/index, `List` vs array | `removesByIndexAndValue`, `listGrowsPastInitialSize` | 6 | 40 | ex |
| 02 | sets | `HashSet`, dedupe, contains: `uniqueWords` | `uniqueWordsDropsDuplicates` | 4 | 30 | win |
| 03 | maps | `HashMap`, `getOrDefault`, `merge`, `computeIfAbsent`: word frequency | `countsWords`, `computeIfAbsentGroupsByFirstLetter` | 10 | 55 | ex |
| 04 | order-and-hashing | HashMap / LinkedHashMap / TreeMap iteration order, hashing basics | (quiz) | 4 | 15 | read |
| 05 | lambdas-and-method-refs | `Function`, `Predicate`, `Runnable`, `String::length` | `appliesPredicate`, `methodRefEqualsLambda` | 8 | 45 | ex |
| 06 | sorting | `Comparator.comparing`, `thenComparing`, `reversed` | `sortsByLengthThenAlpha`, `sortsDescendingByScore` | 8 | 50 | ex |
| 07 | stream-basics | `filter/map/collect/count/limit` pipeline | `filtersEvenSquares`, `sumsWithMapToInt` | 8 | 50 | ex |
| 08 | collectors | `groupingBy`, `counting`, `joining`, `partitioningBy` | `groupsByLength`, `joinsWithCommas`, `partitionsPassFail` | 12 | 65 | ex |
| 09 | immutable-collections | `List.of`, `Map.of`, `copyOf`, `UnsupportedOperationException` | `listOfIsReadOnly`, `copyOfDetaches` | 5 | 30 | win |
| 10 | flatmap-reduce | `flatMap`, `reduce`, `IntStream.rangeClosed`, `max` returning `Optional` | `flattensNestedLists`, `reducesProduct`, `maxOfEmptyIsEmpty` | 12 | 65 | ex |
| 11 | boss-log-analyzer | parse log lines, counts per level, top-3 noisiest sources, errors per minute, all as streams | `parsesLine`, `countsPerLevel`, `topSourcesRanked`, `errorsPerMinuteBucketed` | 50 | 350 | boss |

### World 04 - Exceptions & IO (`04-exceptions-io`)

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | try-catch-finally | catch `NumberFormatException`, `finally` always runs | `parseOrDefaultReturnsFallback`, `finallyRunsOnException` | 6 | 35 | win |
| 02 | checked-vs-unchecked | the hierarchy; why `throws` exists; when to wrap | (quiz) | 4 | 15 | read |
| 03 | custom-exceptions | `InsufficientFundsException` with fields and cause | `carriesAmountShort`, `wrapsOriginalCause` | 8 | 45 | ex |
| 04 | try-with-resources | `AutoCloseable`, close order, suppressed exceptions | `closesInReverseOrder`, `closesEvenWhenBodyThrows` | 10 | 55 | ex |
| 05 | paths-and-files | `Path`, `Files.writeString/readString/exists`, `@TempDir` | `writesThenReadsBack`, `resolvesChildPath` | 8 | 45 | ex |
| 06 | lines-and-streams | `Files.lines`, `BufferedReader`: count lines and words | `countsLines`, `countsWordsAcrossLines` | 10 | 55 | ex |
| 07 | parse-csv | split, trim, quoted commas, bad rows collected | `parsesHeaderAndRows`, `skipsBlankLines`, `reportsBadRowNumber` | 12 | 65 | ex |
| 08 | java-time | `LocalDate`, `Duration`, `DateTimeFormatter`, injected `Clock` | `daysUntilDeadline`, `formatsIsoDate`, `usesInjectedClock` | 8 | 40 | win |
| 09 | properties-files | `Properties`, defaults, typed getters over a `Reader` | `readsKeyWithDefault`, `parsesIntAndBoolean` | 8 | 45 | ex |
| 10 | boss-expense-report | CSV of expenses, validate, monthly totals per category, write report file; bad rows reported, never crash | `readsAllValidRows`, `collectsRowErrors`, `totalsPerCategory`, `writesReportFile` | 50 | 350 | boss |

### World 05 - Generics & Optional (`05-generics-optional`)

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | generic-methods | `<T> T firstOr(List<T>, T)`, `swap`, inference | `firstOrReturnsFallbackOnEmpty`, `swapExchangesElements` | 6 | 35 | win |
| 02 | generic-classes | `Box<T>`, `Pair<A,B>`, a tiny `Stack<T>` | `stackPushPopLifo`, `pairHoldsTwoTypes` | 10 | 55 | ex |
| 03 | bounded-types | `<T extends Comparable<T>>`: `max`, `clamp` | `maxWorksForStringsAndInts`, `clampBoundsAnyComparable` | 8 | 50 | ex |
| 04 | wildcards | `? extends` / `? super` (PECS): `sumAll`, `addNumbers` | `sumAllAcceptsListOfInteger`, `copyIntoSuperTypeList` | 10 | 60 | ex |
| 05 | type-erasure | what erasure means; why `new T[]` and `instanceof List<String>` fail | (quiz) | 4 | 15 | read |
| 06 | optional-basics | `Optional.of/empty/ofNullable`, `orElse` vs `orElseGet`, never `get()` | `findsUserOrEmpty`, `orElseGetIsLazy` | 8 | 45 | ex |
| 07 | optional-chaining | `map`, `flatMap`, `filter`, `ifPresentOrElse` | `chainsThroughNullableFields`, `flatMapAvoidsNestedOptional` | 8 | 50 | ex |
| 08 | custom-functional-interface | `@FunctionalInterface Validator<T>`, `and`/`or` default methods | `andCombinesValidators`, `lambdaImplementsValidator` | 6 | 35 | win |
| 09 | boss-typed-event-bus | `EventBus` with `subscribe(Class<T>, Consumer<T>)` / `publish`, supertype dispatch, unsubscribe handle | `deliversToTypedSubscribers`, `ignoresOtherTypes`, `supertypeSubscribersReceive`, `unsubscribeStopsDelivery` | 50 | 350 | boss |

### World 06 - Build & Test (`06-build-test`)

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | junit-anatomy | `@Test`, `@BeforeEach`, `@DisplayName`, lifecycle per method | (quiz) | 4 | 15 | read |
| 02 | assertj-style | make AssertJ chains pass: `contains`, `hasSize`, `extracting` | `assertsOnLists`, `assertsOnStrings`, `extractsFields` | 8 | 35 | win |
| 03 | write-your-first-test | **mutation-graded**: write tests for `Calculator.divide` | `passesOnCorrectCode`, `catchesDivideByZeroMutant`, `catchesSignMutant` | 12 | 60 | ex |
| 04 | parameterized-tests | fix code so a `@ParameterizedTest` + `@CsvSource` table passes: `fizzbuzz` | `fizzbuzzTable`, `emptyAndNullSources` | 8 | 50 | ex |
| 05 | testing-exceptions | `assertThatThrownBy`, message and cause; write tests for `Account.withdraw` (mutation-graded) | `passesOnCorrectCode`, `catchesMissingOverdraftCheck`, `catchesWrongMessage` | 12 | 65 | ex |
| 06 | test-lifecycle-and-fixtures | `@BeforeEach` vs `@BeforeAll`, `@TempDir`, `@Nested`, `@Tag` | (quiz) | 4 | 15 | read |
| 07 | mockito-stubbing | `mock`, `when/thenReturn/thenThrow`; `Notifier` with a mocked `MailSender` | `sendsWelcomeMailOnSignup`, `fallsBackWhenSenderThrows` | 10 | 60 | ex |
| 08 | mockito-verify | `verify`, `times`, `never`, `ArgumentCaptor`, `InOrder` | `verifiesCallCount`, `capturesSentMessage`, `neverSendsToBlankAddress` | 12 | 65 | ex |
| 09 | gradle-deps-and-tasks | add a `testImplementation` dependency and a `tasks.register("hello")` to `build.gradle.kts`; code needs the lib | `libraryClassResolves`, `helloTaskExists` | 10 | 55 | ex |
| 10 | fakes-vs-mocks | replace a mock with a hand-written in-memory fake; why fakes survive refactors | `inMemoryRepoStoresAndFinds`, `serviceWorksAgainstFake` | 8 | 40 | win |
| 11 | boss-bowling-kata | TDD kata: scorer passes 6 given tests (spares, strikes, 10th frame, 300); plus write 2 own tests (mutation-graded) | `gutterGameIsZero`, `allOnesIsTwenty`, `spareAddsNextRoll`, `strikeAddsNextTwo`, `tenthFrameBonus`, `perfectGameIs300`, `ownTestsCatchMutants` | 60 | 400 | boss |

### World 07 - Concurrency (`07-concurrency`)

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | threads-start-join | `Thread`, `Runnable`, `join`; work off the main thread | `runsOnAnotherThread`, `joinWaitsForResult` | 8 | 45 | ex |
| 02 | race-condition | broken counter, fix with `synchronized`; stress test 8 x 50k | `unsyncedCounterLosesUpdates`, `synchronizedCounterIsExact` | 10 | 60 | ex |
| 03 | memory-model | `volatile`, happens-before, why "works on my machine" lies | (quiz) | 4 | 15 | read |
| 04 | atomics-and-concurrent-maps | `AtomicInteger`, `LongAdder`, `ConcurrentHashMap.merge` | `atomicCounterIsExact`, `concurrentMapCountsWordsAcrossThreads` | 8 | 50 | ex |
| 05 | executor-service | fixed pool, `submit`, `shutdown`/`awaitTermination` | `runsAllTasks`, `poolSizeLimitsParallelism`, `shutsDownCleanly` | 10 | 55 | ex |
| 06 | callable-and-future | `Callable<T>`, `Future.get`, timeout, `invokeAll` | `getsValueFromFuture`, `timeoutThrows` | 6 | 35 | win |
| 07 | completablefuture-chain | `supplyAsync`, `thenApply`, `thenCompose` | `chainsTransformations`, `composeFlattensFutures` | 10 | 60 | ex |
| 08 | completablefuture-combine | `thenCombine`, `allOf`, `exceptionally`, `handle` | `combinesTwoResults`, `allOfWaitsForAll`, `exceptionallyRecovers` | 12 | 70 | ex |
| 09 | virtual-threads | `Thread.ofVirtual()`, `newVirtualThreadPerTaskExecutor`; 10,000 blocking tasks finish fast | `tenThousandBlockingTasksFinish`, `virtualThreadReportsIsVirtual` | 10 | 60 | ex |
| 10 | latch-and-semaphore | `CountDownLatch`, `Semaphore` to cap parallelism | `latchReleasesWhenAllDone`, `semaphoreCapsConcurrency` | 6 | 40 | win |
| 11 | boss-link-checker | check URLs in parallel via an injected fake `Fetcher`, at most N at once, per-link timeout, report keeps input order | `checksAllLinks`, `neverExceedsMaxParallel`, `timeoutMarksLinkDead`, `reportKeepsInputOrder`, `worksOnVirtualThreads` | 60 | 400 | boss |

### World 08 - Spring Boot (`08-spring-boot`)
Standalone MockMvc and slice tests keep runs short. No real server until the capstone.

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | what-spring-does | IoC, beans, the container, what Boot auto-configures | (quiz) | 4 | 15 | read |
| 02 | beans-and-di | `@Component`/`@Service`, constructor injection, `AnnotationConfigApplicationContext` | `contextCreatesBean`, `constructorInjectionWiresDependency` | 10 | 55 | ex |
| 03 | config-and-properties | `@Configuration`/`@Bean`, `@ConfigurationProperties` record, `@Qualifier` | `beanMethodRegistersBean`, `propertiesBindToRecord`, `qualifierPicksNamedBean` | 12 | 65 | ex |
| 04 | rest-controller | `@RestController`, `@GetMapping`, JSON body; standalone MockMvc, no context | `getHelloReturnsJson`, `unknownPathIs404` | 8 | 50 | ex |
| 05 | path-and-query-params | `@PathVariable`, `@RequestParam` with default | `pathVariableIsBound`, `queryParamDefaultsWhenMissing` | 6 | 35 | win |
| 06 | request-body-and-status | `@RequestBody` record, `ResponseEntity`, 201 + `Location` | `postCreatesAndReturns201`, `locationHeaderPointsToNewResource` | 10 | 60 | ex |
| 07 | validation | `@Valid`, `@NotBlank`, `@Min`; 400 on bad input | `blankNameRejectedWith400`, `negativeQuantityRejected`, `validBodyAccepted` | 10 | 60 | ex |
| 08 | exception-handling | `@RestControllerAdvice`, `ProblemDetail`, domain exception to 404 | `missingResourceIs404WithProblemBody`, `validationErrorsListFields` | 10 | 60 | ex |
| 09 | service-layer-webmvctest | `@WebMvcTest` + `@MockitoBean` service; controller stays thin | `controllerDelegatesToService`, `serviceExceptionBecomes404` | 12 | 65 | ex |
| 10 | jpa-entity-and-repo | `@Entity`, `JpaRepository`, `@DataJpaTest` on H2 | `savesAndFindsById`, `generatesId`, `deleteRemovesRow` | 12 | 65 | ex |
| 11 | query-methods | derived queries, `@Query`, paging | `findsByAuthorIgnoreCase`, `customQueryOrdersByYear`, `pagingReturnsRequestedSlice` | 12 | 70 | ex |
| 12 | boss-bookshelf-api | CRUD `/books`: validation, error mapping, JPA on H2, service + controller; MockMvc + `@DataJpaTest` | `createsBook`, `rejectsInvalidBook`, `getsBookById`, `unknownIdIs404`, `listsPagedByAuthor`, `deleteIs204`, `repoPersistsAcrossLayers` | 80 | 450 | boss |

### World 09 - Capstone (`09-capstone`): "habits", a REST service + CLI client
Habit tracker. Server = Spring Boot (reuses 08). CLI = plain Java with `java.net.http.HttpClient`. Everything is tested
without a running server except the boss.

| # | slug | idea | objectives | min | XP | kind |
|---|---|---|---|---|---|---|
| 01 | design | endpoints, JSON shapes, module layout; what each quest builds | (quiz) | 4 | 15 | read |
| 02 | domain-model | `Habit` record, `CheckIn`, streak calculation with injected `Clock` | `streakCountsConsecutiveDays`, `streakResetsAfterGap`, `todayCheckInIsIdempotent` | 12 | 65 | ex |
| 03 | habit-service | in-memory repository behind an interface; unique-name rule | `createsHabit`, `rejectsDuplicateName`, `checkInUpdatesStreak` | 12 | 65 | ex |
| 04 | rest-endpoints | `/habits`, `/habits/{id}/checkins` under MockMvc | `listsHabits`, `postsCheckIn`, `unknownHabitIs404` | 15 | 75 | ex |
| 05 | jpa-persistence | swap in a JPA repo; the same service tests stay green | `jpaRepoSatisfiesSameContract`, `checkInsPersist` | 12 | 65 | ex |
| 06 | cli-parser | `habits add "run"`, `habits list`, `habits done 3` into sealed command records | `parsesAdd`, `parsesDone`, `unknownCommandReportsUsage` | 10 | 55 | win |
| 07 | http-client | CLI talks to a JDK `HttpServer` stub: GET/POST, JSON, error exit codes | `listSendsGetAndPrintsRows`, `addPostsJson`, `serverErrorBecomesExitCode1` | 15 | 75 | ex |
| 08 | boss-ship-it | `@SpringBootTest(RANDOM_PORT)` boots the service, the real CLI client runs against it, `bootJar` builds; then run it yourself (manual victory lap) | `serviceStartsOnRandomPort`, `cliAddsAndListsAgainstRealServer`, `streakShownAfterTwoCheckIns`, `bootJarTaskProducesJar` | 90 | 500 | boss |

Capstone quests 02-07 are the largest non-boss quests (10-15 min, multi-file); keep their XP at the top of the exercise range.

---

## 3. Optional side worlds

- **Kotlin on the JVM.** Worth it, as a ~8-quest "Kotlin for Java devs" world after 03: data classes vs records, null
  safety vs Optional, extension functions, `when`, coroutines vs virtual threads. Same Gradle runner (add the Kotlin plugin,
  JUnit 5 unchanged), ~60 MB extra download, low risk. Useful because Minecraft tooling and KubeJS-adjacent code touch Kotlin. Build after the main track.
- **Quarkus.** Skip for now. It overlaps Spring Boot (DI, REST, JPA) with its own testing model (`@QuarkusTest` boots the
  app; dev services want Docker for databases) and a heavier toolchain. Little new conceptually; revisit only if needed for work.
- **JavaFX.** Low priority, cautious yes. Auto-grade only view-model logic (properties, bindings, `ObservableList`) with plain
  JUnit; real UI tests need TestFX plus a display and are flaky. 6-8 quests ending in an app the learner runs by hand. Adds the
  OpenJFX Gradle plugin and ~50 MB natives. Oxide itself is Tauri, so only worth it if the learner wants desktop apps.

---

## 4. Risks and open questions

**R1. Spring Boot version.** 3.5.16 is the last OSS 3.x (June 2026); Boot 4.1.1 is current (4.0.8 is the last 4.0). Boot 4
modularised starters and test support and moves to Jackson 3, so imports and test artifacts differ. Plan: build 08/09 on
3.5.16 (matches the bulk of tutorials), keep all Spring versions in one shared properties file so a later bump is one change plus
re-verifying ~20 starters (Q3).

**R2. Spring tests on save.** A context start is 2-8 s and the watcher fires on every save. Mitigations: standalone MockMvc for
controller quests (no context), `@WebMvcTest` / `@DataJpaTest` slices only, `@SpringBootTest` only in 09-08,
`spring.main.lazy-initialization=true` in test properties, Gradle config cache, watcher debounce >= 1.5 s, cancel the previous run on a new save.

**R3. Download size.** Gradle 9.8.1 ~130 MB, JUnit+AssertJ ~8 MB, Mockito+ByteBuddy ~6 MB, Spring stack ~70-90 MB, Hibernate/H2 ~25 MB.
About 250-300 MB total, in three moments: setup quest, world 06, world 08. The Setup quest shows the size before downloading.
Sizes are estimates; measure on a clean `GRADLE_USER_HOME`.

**R4. Bosses without a server.** Every boss is a pure library plus a thin boundary: `Fetcher`, `Clock`, `Path` under `@TempDir`,
`MockMvc`, a JDK `HttpServer` on port 0. Only 09-08 binds a port (random). If binding triggers the Windows firewall prompt
or is flaky, fall back to MockMvc + a `HttpClient` fake and keep "run it yourself" manual.

**R5. Probabilistic concurrency tests.** Race tests can pass by luck: use structural checks plus stress, cap runtime ~2 s, always
time out. Virtual-thread timing tests use generous bounds and assert `Thread.isVirtual()`.

**R6. Mockito warnings on JDK 21.** Self-attach and dynamic-agent warnings appear without `-javaagent`; Mockito quests configure it,
and the engine filters those lines from the raw tail.

**R7. Learner-written tests.** Need mutation grading (Q1). If deferred, turn 06-03, 06-05 and the boss's last objective into
"fix the failing tests / complete this test" exercises.

**R8. Windows specifics.** Keep `GRADLE_USER_HOME` short (e.g. `C:\oxide\g`) to stay under 260-char paths; `gradlew.bat` needs
`JAVA_HOME`; antivirus can slow the first run (suggest excluding the Gradle home).

**R9. JUnit 5 vs 6.** Moving to 6.x later changes the BOM version only if annotations stay compatible; verify with content-check before committing to it.

**R10. Test-name hygiene.** Parameterized and `@Nested` tests need the XML rules in section 1. content-check must run each solution
and assert every `test =` name appears green.

Open questions:

- **Q1.** Build mutation-graded "write a test" support in the gradle runner now, or defer those 3 quests (R7)?
- **Q2.** Commit `gradle-wrapper.jar` into every starter (self-contained, ~60 KB each) or inject one shared copy at workspace creation?
- **Q3.** Spring Boot 3.5.16 (stable, out of OSS support) or Boot 4.x for worlds 08-09?
- **Q4.** Ship all 103 quests, or worlds 00-03 first (all Minecraft needs) and the rest later?
