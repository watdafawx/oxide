A sword is COMMON, UNCOMMON, RARE or EPIC, never "legendari" from a typo in a config file. Stored as a `String`, every typo compiles and turns up as a bug in someone's world. You want a type with exactly four values, and nothing else.

**A fixed set.** An **enum** lists every value a type can have: `public enum Rarity { COMMON, UNCOMMON, RARE, EPIC; }`. Those are its **constants**, written in capitals. `Rarity.EPIC` is one of them, and `Rarity.LEGENDARY` does not compile.

**One of each.** Each constant exists exactly once, so `==` is the right way to compare them, unlike text. Inside the enum's own methods, `this` is the current constant, and `this == EPIC` asks "am I epic?".

**Switch over it.** A switch expression from world 01 works on an enum, one case per constant:

```java
String sky = switch (this) {
    case OVERWORLD -> "blue";
    case NETHER -> "red";
    case END -> "black";
};
```

**No default needed.** The compiler knows every constant, so a switch that covers them all needs no `default`. Add a LEGENDARY tier one day and every switch that forgets it stops compiling, which is exactly where you want to find out.

**Enums are classes.** The constants are objects, so an enum can have methods like `upgrade()`, and the next quest gives them fields too.

## Your turn

- In `isCommon`, hand back whether this rarity is `COMMON`.
- In `upgrade`, switch over `this`: each tier goes to the next, and EPIC wraps round to COMMON.
- Save. Both objectives turn green.
