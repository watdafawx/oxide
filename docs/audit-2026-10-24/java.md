# Curriculum audit: `java` track, worlds 00-03 (2026-10-24)

Scope: every `quest.toml` + `lesson.md` (plain flavour) in `content/tracks/java/worlds/{00-setup,01-basics,02-oop,03-collections}` (55 quests). Starters and solutions read for 00/02, 00/06, 00/08, 01/09, 01/13, 01/20, 01/22, 01/24, 02/01, 02/02, 02/12 (Account), 03/01, 03/06. Games flavour lessons skimmed for 00/06, 00/08, 01/24, 02/01, 02/07, 03/01 (they mirror the plain text almost line for line, so the findings apply to both).

Known findings I did not repeat: missing imports (02/08, 02/09, 03/08, 01/24) and the "not longest" quiz fixes are done (PLAYTEST-2026-10-20b-java). The decision "00 never explains `public static`, `class`, `package`" is recorded there as deliberate. This audit argues it should be revisited, because nothing later picks it up properly (see Pattern 1).

**Headline: the track is two different courses.** 00-setup and 01-basics were rewritten to the house style (`docs/REWRITE-java-00-01.md`): one idea per quest, kicker cards, situation, then the thing, then the name. 02-oop and 03-collections were never rewritten. They are the old shape: a `**Why this matters:**` line, then one big code block with 4-8 new concepts, then a bullet glossary, then "Open `src/main/java/...`" (the house style forbids that step). Each quest in 02/03 holds 3-8 new ideas. Worlds 00 and 01 together run 32 quests at one idea each. The term ledger (`docs/tracks/java-terms.md`) stops at 01-basics. A beginner who is fine through quest 32 hits a wall at 02/01.

---

## Scores

| world | Beginner clarity | Experienced-dev value | Coherence / sequencing | Task-lesson fit | Overall |
|---|---|---|---|---|---|
| 00-setup | 6 | 5 | 7 | 7 | **6.5** |
| 01-basics | 7 | 4 | 7 | 7 | **7** |
| 02-oop | 3 | 8 | 5 | 6 | **5** |
| 03-collections | 3 | 8 | 5 | 6 | **5.5** |

---

## 00-setup (8 quests)

**Verdict.** The loop (save, read red, fix, green) is taught well, and the compiler-message reading in 03 is excellent. The world gives the "how" but rarely the "what". The words **method**, **class**, **call**, **value**, **type** and **statement** are used from quest 02 onward and never defined. `public static String greeting()` is waved away as "the shape every method has for now". A non-programmer does not know what a method *is*, why it has a name, or what "the checker calls it" means. `String[] args` is called a **list**, a word that will mean something else in 03/01 (`List`). For an experienced dev it is a fast, harmless on-ramp: the JDK/javac/JVM/Gradle read is useful, the rest is trivial.

**Worst quests**
- `00-setup/02-hello-oxide`: the most important lesson in the track, and it explains the least. "A method that says hello", "the checker calls it", "`public static String greeting()`: copy it as it is" all lean on undefined ideas: what a method is, what calling is, what "hands back" means. The file the learner sees is `public class Hello { ... }`, and `class` is never mentioned (it is first explained in 02-oop/01, 32 quests later). The `/** ... */` comment and `// TODO` are not explained either. One card, "A method is a named recipe; the checker runs it by name and looks at what it hands back", would fix most of this.
- `00-setup/08-boss-first-report`: the starter switches from `return null; // TODO` (used in quests 02-07) to `throw new UnsupportedOperationException("TODO: ...")`. Nothing in the lesson says what `throw` is or that the whole line must be **replaced**. A beginner who adds a `return` under the `throw` gets `unreachable statement`, a compile error never shown. The red message also changes from `expected/but was` to an exception stack trace. This starter idiom then runs through 18 more starters (all of 02 and 03) and is never explained. 03/09 then teaches `UnsupportedOperationException` as "what a read-only list throws", which adds to the confusion.
- `00-setup/06-main-and-args`: `public static void main(String[] args)` is the thing every Java beginner asks about, and the quest titled "main and args" explains only `args`. `void` (taught 01/20), `static`, `String[]` as a type, and `System.out.println` (glossed, fine) are not. The learner also never actually *runs* `java Banner Ada`, so "words from the terminal" stays abstract. `[]` is glossed as "a **list** of texts", but it is an **array** (named in 01/13, taught in 01/18), and `List` is a different thing in 03/01.
- `00-setup/04-project-layout`: "`package oxide.hello_oxide;` says which folder inside it the file belongs to" is vague, and says nothing about why packages exist. Telling learners to read test files is fine, but `assertThat(...).isEqualTo(...)`, `@Test` and `import static` are never glossed. Minor.

**Best quests (models)**
- `00-setup/03-compile-error`: "Three bites" (file:line, message, caret) and "One at a time" are exactly right. They give the reason (later errors are side effects) and a concrete thing to do.
- `00-setup/05-glue-text`: introduces **parameter** with a plain gloss ("I need a piece of text, and inside the method I will call it `name`"), then "Spaces are text too". A model for the beginner voice.

---

## 01-basics (24 quests)

**Verdict.** The best world in the track, and close to the house style. Lessons usually give the *why*: "Why bother" for variables, "Too late" for casting after multiplication, a safe champion start in 19, `==` vs `equals` in 16. Each quest is one idea and the tasks follow from the lessons. Weak points: a read quest about objects and garbage collection sits in the middle of control flow. Arrays are named before they are taught. `throw new ...` and `new` appear as unexplained recipes. `break`/`continue`, comments and reading numbers from text are missing. The boss needs `charAt`, which the rewrite deliberately dropped. For an experienced dev, 24 quests of `int`/`if`/`for` is slow. Nothing lets them skip or test out, though the traps (overflow wrap, `(long) a * b` vs `(long)(a*b)`, `0.1+0.2`, `==` on strings) carry real value.

**Worst quests**
- `01-basics/24-boss-wordle-scorer`: the hints and solution use `guess.charAt(i)` on every line. `charAt` is taught nowhere in 00-01 (REWRITE-java-00-01 confirms it was dropped). The "New tools" block lists `char[]`, `Arrays.fill`, `new String(row)`, `c - 'a'`, but not `charAt`, `--` (decrement; only `++` was taught), `static final` constants (`GREEN`, `YELLOW`, `GRAY` are "the constants in the file"), or `sb.append(switch ...)`. The learner *could* use `toCharArray()` and index it, but no hint says so. Also a pacing cliff: from one-line methods to a two-pass algorithm with a tally indexed by char arithmetic.
- `01-basics/10-how-java-runs` (read): placed between `switch` and the first loop, it talks about "every **object** your program makes", "nothing **refers** to it", and "an old list of every player". Objects, references and lists do not exist yet (objects arrive in 02/01, lists in 03/01). It belongs after 02-oop/01, or at the end of 02.
- `01-basics/09-switch-arms`: the lesson's code and the quiz blank both use `default -> throw new IllegalArgumentException("day " + day);`. "Throwing an error" is the only explanation of `throw`, `new` and exception class names. The ledger marks it "(*starter*)", but the learner is asked to *type* `default` in the quiz with the throw around it. Also two cards end in `:` and lean on the next card ("makes bad input loud instead of silent:"), which breaks "each card must read on its own". 07 does the same.
- `01-basics/13-each-character`: "`text.toCharArray()` hands back every character ... as an **array** like the ones `args` was". In 00/06, `args` was called a "list", and arrays are taught 5 quests later (18). The enhanced-for is taught here on arrays before arrays exist.
- `01-basics/22-build-a-table`: three new ideas in one quest: `StringBuilder` (new, `append`), width flags `%-6s`/`%4d`, and index-based loops over two parallel arrays. The TODO comment gives the exact format string. "It is better than `+` in a loop, which builds a new text every round" is the only "why", and a beginner cannot see why that matters.
- `01-basics/17-flip-it-round`: `new StringBuilder(text).reverse().toString()` is taught as a "recipe" with `new` unexplained. That is acceptable once, but `new` then shows up in 18 (`new int[24]`), 22 and 24 (`new String(row)`), still never explained, until 02/01.

**Best quests (models)**
- `01-basics/03-bigger-numbers`: situation (8 billion people), name (`long`), trap ("Too late. ... A wrong answer, widened"), fix (cast), precision note ("cast sticks to the value right after it"). Good for both audiences.
- `01-basics/16-equals-not-double-equals`: explains *why* `==` fails with a concrete cause (one text from the network, one typed in code), and the checker builds exactly that case.
- `01-basics/19-largest-so-far`: shows the wrong start (0) with a concrete counterexample (-5, -2) before giving the right one.
- `01-basics/12-while-loop`: "It can run zero times", "Change something" (infinite loop), and "Which loop to pick" give the reason for choosing each tool, not only the syntax.

---

## 02-oop (12 quests)

**Verdict.** Strong, modern content for someone who already programs: records, compact constructors, sealed interfaces, pattern switch with guards and record patterns, and the equals/hashCode contract with the bucket explanation. An experienced Java 8 dev learns real things. For a beginner it is a cliff. Every lesson is one code block that introduces 4-8 terms at once (`class`, `field`, `object`, `new`, `private`, `void`, `constructor`, `this`, `final`, `@Override`, `protected`, `abstract`, `Object`, `instanceof` ...), followed by a bullet glossary. Not one quest uses the card/kicker shape, and none is in the term ledger. The world has no `project =` spine in `world.toml`, the plain names are textbook (`Counter`, `Dog extends Animal`, `Employee`, `Shape`, `Pet`), and several lessons depend on `List`, `ArrayList`, `HashSet` and generics from world 03.

**Worst quests**
- `02-oop/01-classes-and-fields`: the most important concept in the track gets one sentence ("a **class**: a blueprint with **fields** ... and **methods**"). The difference between a class and an object, and what `new` does, is a single bullet. The lesson's intro names `Counter` and then shows a class called `Clicker`. The methods suddenly have **no `static`**, after 32 quests of "every method is `public static`", and nothing says why. The code block mixes a class declaration with loose statements (`Clicker a = new Clicker();`), so a beginner cannot tell where each part goes. The constructor is invisible (the default one), so the next quest's word "constructor" lands cold.
- `02-oop/02-constructors-and-this`: the word **constructor** is never defined in 02/01 or 02/02. The lesson opens with "if each constructor repeats the rules". It also introduces `this.x` vs `x` (shadowing), `this(...)` chaining, the first-statement rule, `final`, and `Math.max/min`: five ideas. The starter's "delete the two lines below it" TODO patches a compile trap that the lesson should explain ("`final` fields can only be set once").
- `02-oop/07-polymorphism`: the task is `Payroll.total(List<Employee>)` and the lesson loops over `List.of(new Cat(...), ...)`, but `List`, `List.of` and `<...>` are world-03 material. `protected` appears in the example and is never mentioned. "Picks the child's version at run time" is the whole explanation of polymorphism. The idea that a `Pet` variable can hold a `Cat` was introduced in 05 as one bullet.
- `02-oop/08-equals-hashcode-tostring`: "Three methods from `Object`": the universal superclass `Object` is never introduced. `HashSet`/`HashMap` buckets are used to justify the contract before sets and maps exist (03/02, 03/03). `if (this == o) return true;` has no braces, against 01/07's "Always braces"; 02/03 does the same.
- `02-oop/12-boss-bank-ledger`: needs `ArrayList`, `List.add`, `String.join`, a custom exception, and a switch **statement** with block arms (only switch *expressions* were taught; the hint explains it). It also needs "a rejected transaction changes nothing", which means validate-before-mutate, and that is never taught. The "New tools" block is a band-aid for teaching lists after the boss that needs them.
- `02-oop/03-encapsulation` (read): uses `List<String>`, `new ArrayList<>()`, `List.copyOf`, `throw new IllegalArgumentException`, and "mutable object", all before world 03. Two quiz questions depend on that.

**Best quests (models)**
- `02-oop/10-records`: the opening is the perfect "why" ("last quest you wrote 25 lines ... a record is Java's one-line version"). It builds directly on 08, and the compact-constructor and with-method ideas are motivated.
- `02-oop/11-sealed-and-pattern-switch`: a clear "why" (the compiler counts for you) and exhaustiveness shown in a code comment. Too dense for a beginner, but the model for a dev-facing lesson.
- `02-oop/08`'s contract bullet (hash first, then equals, "not found") is the best "why" paragraph in the world.

---

## 03-collections (11 quests)

**Verdict.** Good, idiomatic modern Java: `List.of` read-only, `remove(int)` vs `remove(Object)`, `merge`/`computeIfAbsent`, `Comparator.comparing().thenComparing().reversed()`, lazy streams, collectors, `copyOf` vs `unmodifiableList`. Experienced devs will find it a crisp refresher with real traps. For a beginner it has the same dense shape as 02, and the hard ideas are glossed in one line each: **generics** (`<T>`, `<>` diamond, "you pick the element type in `<>`"), **boxing** (`Integer` vs `int`: "Box it" is all the reader gets), **lambdas** (used in 03 two quests before 05 teaches them), **functional interfaces**, and **method references** (`Integer::sum`, `TreeMap::new`). Sequencing is off: `03-collections/01-lists` has `requires = []`, so the world is open from day one even though it uses records (06) and the OOP vocabulary. 04 says "You will write your own `equals`/`hashCode` in the OOP world", which reads as if OOP comes *after* (it comes before).

**Worst quests**
- `03-collections/03-maps`: `put`/`get`/`getOrDefault`/`merge`/`computeIfAbsent`/`charAt`/boxing to `Character`/`Map.Entry` iteration, plus a lambda and a method reference with "copy the shape". That is about 8 new ideas, and the TODO-level hints are the full answer. Split it into "count with a map" and "group with a map" after 05.
- `03-collections/11-boss-log-analyzer`: needs `substring(0, 5)` (never taught; REWRITE notes it), `TreeMap::new` (constructor reference, never taught), `Map.Entry.comparingByValue()` with an explicit generic type to make `.reversed()` compile (inference trap, which the lesson works around without explaining), and the 3-argument `groupingBy`. The lesson gives these as a code block to copy.
- `03-collections/01-lists`: generics, `List` vs `ArrayList` ("code that only needs a List keeps working if you swap the implementation" assumes interfaces are understood, but `requires = []` lets a learner skip them), boxing, and `List.of` immutability, all in one quest. Why `List<Integer>` and not `List<int>` is never said.
- `03-collections/05-lambdas-and-method-refs`: "It fits wherever a **functional interface** (one abstract method) is expected" relies on `abstract` (02/07) and interfaces (02/05) in one parenthesis. "Only ones that never change" (effectively final) has no reason given (the lambda may outlive the method). `Predicate`, `Function`, `Runnable` and their method names arrive all at once.
- `03-collections/10-flatmap-reduce`: three unrelated ideas (flatMap, reduce + `IntStream.rangeClosed`, `Optional`). `Optional` is "a box that holds the answer, or is empty", and the TRACKS plan gives it a whole later world (05 Generics & Optional).

**Best quests (models)**
- `03-collections/09-immutable-collections`: one idea, a clear "why" ("who touched my data?" bugs become an immediate exception), and a sharp `copyOf` vs `unmodifiableList` distinction.
- `03-collections/07-stream-basics`: "Nothing runs until the last step ... Until then it is only a plan" is the right mental model in one sentence.
- `03-collections/04-order-and-hashing`: "pick the map whose *promise* matches" is a good framing (once the stale cross-reference is fixed).

---

## Recurring patterns: used before taught (or never taught)

| term / syntax | first used | explained | notes |
|---|---|---|---|
| **method**, **call**, "hands back", **value** | 00/02 | never defined (only used) | the core vocabulary of the whole track |
| `class` (`public class Hello`) | 00/02 (every starter) | 02/01, one sentence | 32 quests later |
| `public` / `static` | 00/02 | `static` 02/04, `public` never explicitly ("public method" vs `private` field in 02/01) | 02/04 never links back: "this is why all your methods so far were `static`: there was no object" |
| `void` | 00/06 (`main`) | 01/20 | 14 quests of `public static void main` unexplained |
| `String[]` called a "list" | 00/06 | arrays 01/18; `List` 03/01 | word clash; call it an array from the start |
| `//` and `/** */` comments | 00/02 starter, 01/01 code | never | trivial, but a non-programmer needs it |
| **type** | 00/07 ("has the type `int`") | never as a concept | why Java makes you write types at all is never said |
| **statement**, **block**, **expression** | 00/03, 01/07, 01/06 | glossed by example only | |
| `throw new XxxException(...)` | 00/08 starter, 01/09 lesson | never (world 04 is Exceptions) | 18 starters use the throw-TODO idiom, unexplained |
| `new` | 01/17 (`new StringBuilder`) | 02/01 ("builds an object") | also 01/18, 01/22, 01/24 |
| **object**, **reference** | 01/10, 01/16 ("same piece of text") | 02/01 (objects), references never | |
| `Integer.MAX_VALUE`, `Math.PI` (`Class.member` dot syntax) | 01/02, 01/14 | static access 02/04 | |
| `charAt` | 01/24 hints + solution | 03/03 (one bullet) | boss needs it |
| `--` decrement | 01/24 solution | never | |
| `static final` constant | 01/24 starter | `final` 02/02, constants never | |
| **constructor** | 02/02 | never defined | |
| `@Override` / annotations | 02/06 | "marks a method", no annotation concept | `@Test` seen since 00/02 |
| `protected` | 02/07 | never | |
| `Object` (root class) | 02/08 | never | |
| `List`, `ArrayList`, `<T>` | 02/03, 02/07, 02/12 | 03/01 (generics one line) | |
| `HashSet` / `HashMap` | 02/08 | 03/02, 03/03 | |
| boxing `int`/`Integer`, `char`/`Character` | 03/01 | one line each | |
| lambda, method ref | 03/03 | 03/05 | admitted "copy the shape" |
| `substring` | 03/11 | never | |
| constructor ref `TreeMap::new` | 03/11 | never | |
| `Optional` | 03/10 | one line | |
| braces rule | 01/07 says always | 02/03, 02/08 break it | inconsistent |

Plus `docs/tracks/java-terms.md` has no rows for 02 or 03, so the ledger rule in CONTENT.md is not applied there at all.

---

## Prioritized improvements

1. **Rewrite 02-oop and 03-collections to the golden shape** (as done for 00-01): one idea per quest, kicker cards, ledger rows. Expect 12 → ~22 and 11 → ~20 quests. Suggested splits: 02/01 → "an object remembers" (fields, `new`, no `static`) + "how an object gets built" (constructor); 02/02 → `this.x` + `this(...)`; 02/07 → abstract + polymorphic loop; 03/01 → generics/`List` + boxing trap; 03/03 → count + group (after lambdas); 03/10 → three quests. Give 02 a `project =` (the TRACKS spine: "modpack updater" / status bot) and make plain names less textbook-ish.
2. **Fix ordering.** Move lists (03/01-02) before 02/03, 02/07 and 02/12, or make 02 avoid `List` entirely (use arrays, which are taught). Move 01/10 (JIT/GC) to after 02/01. Drop `requires = []` on 03/01, or prove 03 stands alone. Fix 03/04's "You will write ... in the OOP world" back-reference.
3. **Add a "What is a method/class, and why `public static`?" card set in 00.** Not a glossary, three cards. In 00/02, define method and call. In 00/06, give the `main` line one card per word, with a promise of where each word is taught properly. In 02/04, add the payoff card: "every method you wrote in 00-01 was `static` because there was no object to ask; now there is". This is the single biggest beginner complaint about Java, and the track currently says "do not worry about it" and never comes back.
4. **Explain the throw-TODO starter idiom once** (00/08, the first time it appears): "`throw` stops the method with an error; delete the whole line and write your `return`". Or keep `return null;`/`return 0;` TODOs through world 01, as 01 already does.
5. **Close the 01/24 boss gaps:** add `charAt(i)` (and `--`, `static final`) to "New tools", or better, add the "Pick a letter" quest (`charAt`, `substring`) that REWRITE-java-00-01 already proposed. It also serves 03/03 and 03/11.
6. **Rename "list" to "array" in 00/06**, and call arrays arrays from then on.
7. **Teach `new` and `throw` as one-card concepts** in 01/17 and 01/09, not as recipes.
8. **Add the missing basics:** `break`/`continue`, `//` comments, `Integer.parseInt` ("Text to number", already proposed), and a short "why types?" card (in 01/01: "Java checks before running that you never put text where a number belongs; that is why you write `int`").
9. **Experienced-dev path:** a "test out" boss or a skip marker for 00-01 (Java devs will not do 24 int/loop quests). In 02/03, keep the dense "dev notes" as an optional final card, so the beginner cards stay light without losing the depth.
10. **Small consistency fixes:** 02/01 `Counter` vs `Clicker` naming; braces in 02/03 and 02/08 examples; "Open `src/main/java/...`" steps in all of 02/03 (house style forbids them); cards that end in `:` (01/07, 01/09, 01/14).

### Example rewrite: `02-oop/01-classes-and-fields` (plain), in the house shape

> A button counter has to remember a number between presses. Every method you have written so far forgot everything the moment it returned. Today's code keeps a number alive between calls.
>
> **A thing that remembers.** So far a method got its values, worked, and handed one back. Now you want a counter that is still 2 after you pressed it twice. That needs a **field**: a variable that lives inside a thing instead of inside one method call.
>
> **The plan and the thing.** A **class** is the plan: "a counter has a number called `count`, and can `increment` it". An **object** is one real counter built from that plan. `new Counter()` builds one. Build two, and each has its own `count`.
>
> **Why no `static` today.** Every method in worlds 00 and 01 was `static`: it belonged to the class and needed no object. `increment()` is different. It changes *this* counter's `count`, so it is written without `static`, and you call it on an object: `c.increment()`.
>
> **Private means mine.** `private int count;` means only the counter's own methods may touch `count`. Other code has to ask through `value()` and `increment()`, so nobody can set it to -3.
>
> ```java
> public class Counter {
>     private int count;              // a field: starts at 0
>
>     public void increment() {       // no static: works on one counter
>         count++;
>     }
> }
> ```
>
> ## Your turn
> - In `value`, hand back the `count` field (delete the `throw` line, write your `return`).
> - In `increment`, add one to `count`.
> - Save. Both objectives turn green.

(The constructor moves to its own quest 02: "What runs when you write `new`".)

---

## What a true non-programmer needs that is missing (input for a "general programming for beginners" world)

The Java track assumes these mental models without ever building them:

1. **What a program is and what "running" means.** It executes top to bottom, one statement at a time, and the computer does exactly what is written. 00 jumps straight to "the checker calls your method".
2. **Values, variables and memory as boxes with labels**, and that `x = x + 1` is not maths. 01/01 covers `=` well, but reassignment (`total = total + 10`) is only shown as shorthand.
3. **Functions as named recipes:** inputs (parameters), output (return), calling, and the difference between *printing* a value and *returning* it. This is a classic beginner confusion, and `main` + `println` vs `return` sits right in 00/06.
4. **Tracing code by hand:** a "predict the output" habit with a table of variable values per line. It would make loops (01/11-12) and the boss algorithm (01/24) far less scary.
5. **Types as a concept:** why a language cares whether something is a number or text, static vs dynamic typing, and what a compile error is *for*.
6. **Reading an error / stack trace** beyond the two messages taught (`expected/but was`, `';' expected`): exceptions, line numbers in stack traces, `NullPointerException`.
7. **Comments, naming and indentation** as tools for humans.
8. **Decomposition:** splitting a problem into small methods. The bosses rely on it ("work in order"), but it is never taught as a skill.
9. **What tests are and why they exist.** The learner lives inside JUnit/AssertJ from quest 2, and the idea "a test is code that checks code" gets only one sentence in 00/04.
10. **Objects as "data + behaviour" before the syntax:** a real-world analogy plus the difference between a reference and a copy (needed for 01/16, 01/20 "arrays are shared", 02/03 leaks, 02/08 identity vs equality). The track touches identity three times, in three different metaphors, and never builds the model.
11. **Algorithms vocabulary:** accumulator, counter, champion/max, swap, two passes. 01/19-20 and 01/24 teach these well in passing. A beginner world could name them as reusable patterns.
12. **Basic CLI and file-system literacy:** a folder, a path, a terminal, what "run `java Banner Ada`" physically means. 00/04 and 00/06 assume it.
