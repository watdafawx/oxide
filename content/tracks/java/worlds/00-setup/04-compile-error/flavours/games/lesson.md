**Why now:** your bot will report the total players across two servers, survival and creative. A bot that does not compile never comes online, so first learn to read what the compiler tells you.

This starter **does not compile on purpose**, so the objective stays grey until it does. Gradle prints errors like this:

```
src/main/java/oxide/compile_error/Players.java:6: error: ';' expected
        String total = survival + creative
                                          ^
```

- **file:line** tells you where to look. The `^` points at the exact spot.
- Fix the **first** error only, then save. Later errors are often side effects, and new ones can appear once the first is gone.
- Types matter: two `int` player counts added together make an `int`. Storing that in a `String` is a **type mismatch**: `incompatible types: int cannot be converted to String`.

## Your turn

- Open `src/main/java/oxide/compile_error/Players.java`.
- Fix the first error, save, read the next one, and fix that too.
- When it compiles, the objective can turn green.
