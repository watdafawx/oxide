Last quest the code ran and the checker said red. This time the code will not even build: the compiler stops first and tells you exactly where. You get to read that message once, with one mistake to fix.

**Build before check.** Before the checker can run, Java has to **compile** your code, which means translating it. If a line breaks Java's grammar, the compile stops and nothing gets checked. That is why the objective stays grey instead of red.

**The message.** The compiler prints what went wrong:

```text
src/main/java/oxide/compile_error/Greeter.java:7: error: ';' expected
        return "Welcome to Oxide!"
                                  ^
```

**Three bites.** The file and line number (`Greeter.java:7`) say where to look. The words after `error:` say what is wrong. The `^` marks the exact spot. Gradle prints the same error twice, once as it happens and once in its summary. That is one error, not two.

**The missing piece.** Every statement in Java ends with `;`, like a full stop at the end of a sentence. The `return` line is missing its stop:

```java
return "Welcome to Oxide!";
```

**One at a time.** Fix the first error, save, and read again. Later errors are sometimes side effects of the first, and fixing it can clear them all.

**If this is new.** Programming 101 explains error messages from zero: [Read the red](/quest/basics/01-what-a-program-is/05-read-the-red).

## Your turn

- Save and read the error: which line is it, and what is missing?
- Add the missing `;` at the end of that line.
- Save again. When it compiles, the objective turns green.
