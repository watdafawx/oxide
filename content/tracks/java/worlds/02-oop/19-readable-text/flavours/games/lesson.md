A player reports a bug and your log says `chest opened at BlockPos@1b2c3d`. That last part is useless: it is the class name and a number, all `Object` knows how to say about your object. The third method from `Object` fixes it.

**What Object says.** `toString()` turns an object into text. The version every class gets from `Object` gives the class name, `@`, and a number. Override it, and your object describes itself.

**Used everywhere.** Java calls `toString()` for you whenever an object meets text: `"Chest at " + pos`, `System.out.println(pos)`, chat messages, and the checker's red messages. Write it once, and every log line and failed test reads clearly.

**Read like chat.** Players read coordinates as `(10, 64, -3)`, so that is the format. It is plain text joining with `+`, the fields in order.

**In code.** Here a chunk position describes itself:

```java
@Override
public String toString() {
    return "chunk " + x + "," + z;      // chunk 3,-7
}
```

## Your turn

- In `toString`, hand back the three numbers in brackets, with `", "` between them.
- Save. Both objectives turn green.
