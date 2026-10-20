**Why this matters:** red is not failure, it is a message. Real programs break all day; the skill is reading *what* broke in ten seconds instead of staring.

A failing assertion looks like this:

```
org.opentest4j.AssertionFailedError:
expected: "Hello, Ada!"
 but was: "Hello, Ada"
    at HelloTest.greetsByName(HelloTest.java:12)
```

- **expected** is what the test wants. **but was** is what *your* code returned. Compare them character by character: here the `!` is missing.
- The last line says which test and **file:line** failed. Open it to see exactly what was checked.
- An unwritten starter method usually returns a placeholder (`null`, `0`, `false`), so a fresh quest shows `but was: null` or `but was: 0`. That just means "not written yet". (A `TODO` exception means the same.)
- A *compile error* is different: the code never ran. You will meet one in the very next quest.

Read the red from the top. Fix one thing, save, read again.

## Your turn

Answer the three questions below.
