**Why now:** the updater has to ask "is this mod outdated?" about hundreds of mods. A `Predicate<Mod>` captures that question once, and every list operation after this can reuse it.

A **lambda** is a small unnamed function: `parameters -> result`. It fits wherever a **functional interface** (one abstract method) is expected. `Mod` is a record with `id`, `name`, `loader`, `downloads`, `version`:

```java
Predicate<Mod> fabricOnly = mod -> mod.loader().equals("fabric");   // Mod -> boolean
Function<Mod, Integer> pop = mod -> mod.downloads();                // Mod -> Integer
Runnable log = () -> System.out.println("checking...");             // no input, no output

fabricOnly.test(sodium);   // true
pop.apply(sodium);         // 9500
```

- `Predicate<T>`: `test` returns boolean. `Function<T, R>`: `apply` turns a T into an R. `Runnable`: `run`, nothing in or out.
- A lambda can read local variables and parameters, but only ones that never change after being set (like `latestById` below).
- When the lambda only calls one existing method, use a **method reference**: `mod -> mod.name()` is `Mod::name`.
- A method that *returns* a lambda is normal: `isOutdated(latest)` hands you a ready-made `Predicate<Mod>`.

## Your turn

- Open `src/main/java/oxide/lambdas_and_method_refs/Updates.java`.
- `isOutdated`: true when the mod's version is not the one `latestById` lists for its id.
- `nameByLambda` and `nameByMethodRef`: return a `Function<Mod, String>` written both ways.
