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
