The organiser's chat command prints the pack as one line, `[Sodium, Iris Shaders, Just Enough Items]`. Building that with `+=` in a loop means a special case for the comma after the last mod. One collector does it without one.

**joining.** `collect(joining(", "))` glues a stream of `String`s into one text with `", "` between items, never after the last one. The items must already be text, so `map` the mods to their names first.

**Prefix and suffix.** `joining(", ", "[", "]")` takes the separator, then text for the front, then text for the end. With no items at all you still get the front and the end: `"[]"`, which is exactly what an empty pack should show.

```java
String path = List.of("config", "sodium", "options").stream()
        .collect(joining("/", "/", ".json"));     // "/config/sodium/options.json"
```

**For the curious.** Inside, `joining` uses a `StringBuilder`, the same tool you used for the table in the Basics world. `String.join(", ", names)` does the plain separator case without a stream.

## Your turn

- In `packLine`, map the mods to their names, then join them with `", "` between, `[` in front and `]` at the end.
