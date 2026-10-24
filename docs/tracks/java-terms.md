# Java track: term ledger

Every term, keyword and piece of syntax, in the order the track introduces it, with the quest
that does. A lesson may use only rows at or above its own quest. Add a row when you introduce
something. Helpers a starter contains (marked *starter*) are shown, not taught.

| term | quest | how it is introduced |
|---|---|---|
| JDK, JDK 21 | 00-setup/01-whats-a-jdk | "the kit" |
| `javac`, compiler, bytecode, `.class` | 00-setup/01-whats-a-jdk | "the translator" |
| `java`, JVM | 00-setup/01-whats-a-jdk | "the runner", with a bash block |
| Gradle, `gradlew`, wrapper, first download | 00-setup/01-whats-a-jdk | "skip the typing" |
| checker (test file), objective, red/green, `expected` / `but was`, `null` | 00-setup/02-hello-oxide | save once and read the red |
| `public static String name()` (named "the shape every method has for now", not explained) | 00-setup/02-hello-oxide | copy the shape |
| `return`, text in double quotes, `;` ends a line | 00-setup/02-hello-oxide | hand the greeting back |
| compile error, `file:line: error:`, `^`, `';' expected`, statement | 00-setup/03-compile-error | missing `;` |
| `src/main/java`, `src/test/java`, `build/`, `build.gradle.kts`, `package` line | 00-setup/04-project-layout | the folder tree |
| parameter (`String name`), several parameters | 00-setup/05-glue-text | greet by name |
| `+` joins text, spaces are text | 00-setup/05-glue-text | `"Hello, " + name` |
| `main`, `System.out.println` (*starter*), `String[] args`, list of texts, `args[0]`, counting from 0 | 00-setup/06-main-and-args | run `java Banner Ada` |
| `String.format`, slot `%s`, slot `%d`, `int` as the whole-number type (in a parameter) | 00-setup/07-fill-the-slots | status lines from a template |
| calling one of your methods from another, `\n` in text | 00-setup/08-boss-first-report | `shout(twice(word))` |
| variable, `int survival = 12;`, `=` stores, `-` and `*`, `int` as a return type | 01-basics/01-a-name-for-a-count | two player counts get names |
| `Integer.MAX_VALUE`, `Integer.MIN_VALUE`, overflow (wraps, silently) | 01-basics/02-numbers-that-wrap | add 1 to the top |
| `long`, `L` suffix, `_` in big numbers, cast `(long)`, widening too late | 01-basics/03-bigger-numbers | player-seconds past 2.1 billion |
| `/` on two ints (fraction thrown away), `%` remainder | 01-basics/04-integer-division | pages of players |
| `double`, decimal literal `2.0`, cast `(double)`, doubles are approximate | 01-basics/05-fractions-with-double | how full is the server |
| `boolean`, `true` / `false`, `< > <= >= == !=`, `&& \|\| !`, `boolean` parameter | 01-basics/06-true-or-false | is it full, may they join (`==` is for numbers; text comes in 16) |
| `if`, `else`, `else if`, return early, braces | 01-basics/07-if-and-else | clamp a player count, trend |
| ternary `cond ? a : b` | 01-basics/08-the-one-line-choice | ONLINE or OFFLINE |
| switch expression, `case x ->`, labels sharing an arm, `default`, bucket by dividing, `throw new IllegalArgumentException(...)` (*starter*), `isLeap` helper (*starter*) | 01-basics/09-switch-arms | ping grades, month lengths |
| JIT, garbage collector, memory leak | 01-basics/10-how-java-runs | read: two helpers inside the JVM |
| counted `for (start; keep going; step)`, `i++`, `+=`, running total, off-by-one, `+=` on text | 01-basics/11-counted-loop | 1 + 2 + ... + N |
| `while`, infinite loop | 01-basics/12-while-loop | halve a retry delay |
| `char`, `'a'`, `toCharArray()`, enhanced `for (char c : ...)`, `c == 'a'` | 01-basics/13-each-character | count colour codes |
| overloading (same name, different parameters), `Math.PI` (*TODO*) | 01-basics/14-same-name-different-inputs | `show(int)`, `show(boolean)` |
| method call on text, `trim()`, `toUpperCase()`, `toLowerCase()`, `length()`, immutable, chaining | 01-basics/15-text-never-changes | tidy a MOTD |
| `equals`, `equalsIgnoreCase`, `==` on text asks "same object" | 01-basics/16-equals-not-double-equals | did the MOTD change |
| `new StringBuilder(text).reverse().toString()` (a recipe), `replace` | 01-basics/17-flip-it-round | palindromes |
| array `int[]`, `{7, 3, 9}`, index from 0, `.length`, `new int[24]`, out-of-bounds error | 01-basics/18-an-array-of-values | hourly samples |
| enhanced `for` over an array, the "champion so far" pattern | 01-basics/19-largest-so-far | busiest hour |
| arrays are shared, `void`, swap with `tmp`, `length - 1 - i` | 01-basics/20-flip-in-place | newest first |
| varargs `int... nums` | 01-basics/21-any-number-of-values | total any number of servers |
| `StringBuilder` with `append`, `%-8s` and `%3d` widths, loop over two arrays by index | 01-basics/22-build-a-table | the server table |
| text block `"""`, `.formatted(...)`, indentation, final newline | 01-basics/23-text-blocks | the status JSON |
| `char[]`, `new char[5]`, `row[2] = 'G'`, `Arrays.fill`, `new String(char[])`, `c - 'a'`, `int[26]` tally, `tally[i]++`, `switch` on a char | 01-basics/24-boss-wordle-scorer | the codeword scorer |
| method (a named list of steps), call, "the call is replaced by what came back", class as the box around methods | 02-oop/01-methods-and-classes | read: names the words used since 00-setup/02 |
| field, class (the plan), object (one thing built from it), `new Counter()`, number fields start at 0, method without `static` called on an object (`front.increment()`) | 02-oop/02-an-object-remembers | a door counter that stays at 2 |
| starter idiom `throw new UnsupportedOperationException("TODO: ...")`: replace the whole line | 02-oop/02-an-object-remembers | "the TODO line" card |
| `private` field, `error: ... has private access`, `public` (anyone with the object may call), check-then-change in a method | 02-oop/03-private-means-mine | a piggy bank that never holds -3 |
| constructor (class's name, no return type, runs once inside `new`), default no-argument constructor | 02-oop/04-what-new-runs | an account must open with an owner |
| `this`, `this.owner = owner`, a plain name means the nearest one (parameter shadows field) | 02-oop/05-this-is-the-field | `owner = owner;` compiles and does nothing |
| `final` field (set once in the constructor), `cannot assign a value to final variable` | 02-oop/06-set-once | the owner never changes; the balance does |
| overloaded constructors, `this(...)` as the first line, `Math.max` (*starter*/comment) | 02-oop/07-many-ways-to-build | short constructors hand over to the full one |
| `static` field (one copy for the class), `Integer.MAX_VALUE` / `Math.PI` read as static fields | 02-oop/08-shared-by-every-object | ticket ids 1, 2, 3; non-static first gives all 1 |
| `static` method called on the class, no `this` inside, factory, `private` constructor; why 00-01 methods were `public static`; why `main` is static | 02-oop/09-no-object-needed | `Ticket.of(...)` |
| encapsulation, returning a private array leaks it, `array.clone()` | 02-oop/10-dont-hand-out-the-array | read: a playlist getter |
| `interface`, `implements`, method lines without bodies, interface as a variable type, `@Override`, `does not override abstract method` | 02-oop/11-a-shared-promise | every shape has a name and an area |
| `default` method in an interface, calling the interface's own methods, `Math.round` (*TODO*) | 02-oop/12-a-default-body | `describe()` written once |
| `extends`, parent / child class, inheritance, overriding, parent code runs the child's version, one parent only | 02-oop/13-is-a | a Dog's sound |
| `super(...)` in a constructor, `super.describe()` | 02-oop/14-the-parents-version | build on the parent's text |
| `abstract` class (`cannot be instantiated`), `abstract` method | 02-oop/15-a-half-finished-parent | what is a plain employee's pay? |
| array of objects (`Employee[] staff = {...}`), polymorphism (one call, each object's own version) | 02-oop/16-one-loop-every-kind | payroll that survives a new kind |
| `Object` (root class), identity vs equality by value, overriding `equals(Object o)`, `o instanceof Money other`, reading another object's private field in the same class | 02-oop/17-equal-by-value | two 12.50 EUR objects |
| `hashCode`, the equals/hashCode contract, `HashSet` (one card: holds each value once, buckets by hash; world 03 teaches it), `Objects.hash`, `import java.util.Objects;` (*starter*) | 02-oop/18-hash-matches-equals | the set holds the same price twice |
| `toString`, Java calls it when an object meets text, `%02d` | 02-oop/19-readable-text | `Money@1b2c3d` in a red message |
| `enum`, constants, `==` on constants, `switch (this)` with no `default` | 02-oop/20-a-fixed-set | traffic light colours |
| enum fields and constructor `RED(30)`, constants end with `;`, `values()`; `valueOf`, `ordinal` (for the curious) | 02-oop/21-constants-with-data | each light's seconds |
| `record`, components, generated constructor / accessors / equals / hashCode / toString, accessor without `get`, immutable | 02-oop/22-data-in-one-line | thirty lines become one |
| compact constructor, assigning a component to normalise, `throw`, exception, `new IllegalArgumentException("...")`; ties back to the TODO line | 02-oop/23-reject-bad-values | a parcel of -1 grams |
| `with...` method returns a changed copy, the copy passes the constructor's rules | 02-oop/24-change-means-copy | `withGrams` |
| `sealed interface ... permits`, `does not cover all possible input values`, type pattern `case Circle c ->`, records implementing an interface | 02-oop/25-the-compiler-counts | area of three shapes, no default |
| guard `case X x when ... ->`, `dominated by a preceding case label` | 02-oop/26-a-case-with-a-condition | a circle of radius 0 is a dot |
| record pattern `case Rectangle(double w, double h) ->`, nested record patterns (games) | 02-oop/27-unpack-in-the-case | is it a square? |
| switch statement (arms do things), block arm `-> { ... }`, `-=`, record with no components `Reset()` | 02-oop/28-a-switch-that-does | thermostat commands |
| own exception class `extends RuntimeException` with `super(message)`, throw before any change, do the step that can fail first | 02-oop/29-check-then-change | a refused payment changes neither wallet |
| `Arrays.copyOf` (*starter* helper `record`), `StringBuilder` statement | 02-oop/30-boss-bank-ledger | the bank / chest ledger |
| `ArrayList`, `add`, `size()`, `get(i)`, `import java.util.ArrayList;` (*starter*), type parameter `<String>`, generics (why: the compiler checks every add/get), diamond `<>` | 03-collections/01-a-list-that-grows | arrays cannot grow; a list that makes room |
| `List` as an interface, `List<String>` on the left / as a parameter, `List.of(...)` (seen, not explained), enhanced `for` over a list, `incompatible types` compile error | 03-collections/02-ask-for-a-list | the checker passes a `List.of` list to an `ArrayList` parameter (starter fails to compile) |
| primitive vs object ("reference"), `List<int>` error `unexpected type`, wrapper classes `Integer` `Long` `Double` `Character` `Boolean`, boxing, unboxing, unboxing `null` → `NullPointerException`, `equals` not `==` on `Integer` | 03-collections/03-numbers-need-a-box | `List<int>` does not compile |
| `remove(int index)` vs `remove(Object)`, `Integer.valueOf`, copy with `new ArrayList<>(other)` (*starter*) | 03-collections/04-remove-by-slot-or-value | slot 1 or ticket/project 1? |
| `List.of` read-only, `UnsupportedOperationException`, `List.copyOf` snapshot, `Set.of` / `Map.of` / `copyOf` (named), `Collections.unmodifiableList` view (curious) | 03-collections/05-lists-that-cannot-change | why the starters copied |
| `Set`, `HashSet`, `add` returns `boolean`, `contains`, `new HashSet<>(list)`, no order | 03-collections/06-sets-hold-each-once | repeated tags / enchantments |
| `Map<K, V>`, `HashMap`, key, value, `put` (replaces), `get` → `null` when missing, `getOrDefault`, counting with `put(k, getOrDefault(k, 0) + 1)`, `containsKey` (curious) | 03-collections/07-maps-key-to-value | price / latest version by name |
| `entrySet()`, `Map.Entry<K, V>`, `getKey()`, `getValue()`, `keySet()`, `values()`, champion with `Integer.MIN_VALUE` start | 03-collections/08-walk-a-map | top scorer / most downloaded |
| `LinkedHashMap`, `TreeMap`, `LinkedHashSet`, `TreeSet`, `firstKey()`, `hashCode()` buckets, equals/hashCode rule | 03-collections/09-who-keeps-the-order | read: three promises about order |
| lambda `x -> ...`, `Predicate<T>`, `test`, `removeIf`, `java.util.function` import (*starter*), functional interface, effectively final (why: the lambda may outlive the method) | 03-collections/10-a-rule-as-a-value | hand `removeIf` a rule |
| `Function<T, R>`, `apply`, method reference `Type::method`, static method reference (`Integer::parseInt` glossed: text to number) | 03-collections/11-method-references | a word to its length / a mod to its name |
| `merge(key, value, rule)`, `Integer::sum` | 03-collections/12-count-with-merge | totals per item / per loader |
| `computeIfAbsent(key, k -> new ArrayList<>())`, nested generics `Map<K, List<V>>` | 03-collections/13-group-into-lists | piles by length / names by loader |
| `Comparator`, `list.sort`, `Comparator.comparing`, `comparingInt`, natural order, `reversed()`, sort a copy | 03-collections/14-sort-by-a-key | leaderboard / popular list |
| `thenComparing`, where `reversed()` goes, `Comparator.naturalOrder()`, stable sort (curious) | 03-collections/15-break-ties | equal scores / same loader |
| stream, `.stream()`, `filter`, `toList()` (read-only result), source unchanged, lazy until the last step, use once, `parallelStream` (curious) | 03-collections/16-your-first-stream | the loop with a temporary list, as a pipeline |
| `map`, `Stream<T>` type follows the step, `limit(n)`, step order | 03-collections/17-change-each-item | names and the podium / top three |
| `mapToInt`, `IntStream`, `sum()`, `max()`/`average()` (named), `count()` returns `long`, `reduce` (curious) | 03-collections/18-numbers-from-a-stream | totals and counts |
| `collect`, collector, `Collectors.groupingBy`, downstream collector, `counting()` (`Long` values), `import static` | 03-collections/19-group-a-stream | the computeIfAbsent loop in one line |
| `Collectors.joining(sep, prefix, suffix)`, `String.join` (curious) | 03-collections/20-join-into-text | `[milk, eggs, bread]` |
| `Collectors.partitioningBy`, both keys always present | 03-collections/21-split-in-two | pass/fail, outdated/current |
| `List<List<T>>`, `flatMap`, `List::stream` | 03-collections/22-flatten-lists | orders / instances into one list |
| `stream.max(comparator)`, `Optional<T>`, empty, `isPresent`/`get` (named), `Optional.map`, `orElse`, `OptionalInt` (curious) | 03-collections/23-maybe-there-is-none | the best of no players |
| `split(sep)` → `String[]`, `List.of(array)`, `split(sep, limit)`, regex separator (curious) | 03-collections/24-cut-a-line | bridge for the boss: cut a log / pack line |
| `substring(0, 5)` (*TODO*), `Integer.parseInt` (*TODO*, games), `new TreeMap<>(map)` sorted copy, `SourceCount` record (*starter*) | 03-collections/25-boss-log-analyzer | log analyzer / update report |
