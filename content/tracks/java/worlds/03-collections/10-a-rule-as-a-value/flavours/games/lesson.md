The pack is now a list of `Mod` records: id, name, loader, downloads and version, read with `mod.version()` and friends. Before an update, the organiser must drop every mod that is not on its latest version. Lists can do that in one call, `removeIf`, but only if you can hand it the rule "is outdated". A rule is code, not a value, and today you write code as a value.

**Hand over a rule.** `mods.removeIf(rule)` removes every item the rule says yes to. The list does the loop; you only supply the question to ask about each mod.

**The arrow.** A **lambda** is a small unnamed method written in one line: `mod -> mod.downloads() < 100`. Left of the arrow is the parameter, right of it is the result. Read it as "given a mod, answer whether it has under 100 downloads".

**Its type.** A lambda that answers yes or no about a `Mod` is a `Predicate<Mod>`. You can store it in a variable, return it from a method, and ask it about one mod with `test`: `rule.test(sodium)` is `true` or `false`.

**Why it works.** `Predicate` is an interface with a single method, `test`. A lambda is the short way to write an object that keeps that one-method promise. Java calls such an interface a **functional interface**.

```java
Predicate<String> isFabric = loader -> loader.equals("fabric");
isFabric.test("forge");     // false
List<String> loaders = new ArrayList<>(List.of("fabric", "forge", "fabric"));
loaders.removeIf(isFabric); // [forge]
```

**Using outside values.** A lambda can read the method's parameters and variables, like `latest` in `mod -> ... latest.get(mod.id()) ...`, as long as they never change after being set (**effectively final**). The reason: the lambda may run long after the method has returned, so Java hands it a copy of the value. If the variable could still change, that copy would silently be out of date.

## Your turn

- In `isOutdated`, return a `Predicate<Mod>` lambda that is true when the mod's version does not equal `latest.get(mod.id())`. Compare text with `equals`.
- In `upToDate`, copy the list, call `removeIf` on the copy with your `isOutdated(latest)` rule, and return the copy.
