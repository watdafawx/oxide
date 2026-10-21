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
