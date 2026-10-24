A shopping app shows the basket as one line, `[milk, eggs, bread]`. Building that with `+=` in a loop means a special case for the comma after the last item. One collector does it without one.

**joining.** `collect(joining(", "))` glues a stream of `String`s into one text with `", "` between items, never after the last one. The items must already be text, so `map` them to `String`s first when they are not.

**Prefix and suffix.** `joining(", ", "[", "]")` takes the separator, then text for the front, then text for the end. With no items at all you still get the front and the end: `"[]"`, which is exactly what an empty basket should show.

```java
String path = List.of("home", "ada", "notes").stream()
        .collect(joining("/", "/", ""));     // "/home/ada/notes"
```

**For the curious.** Inside, `joining` uses a `StringBuilder`, the same tool you used for the table in the Basics world. `String.join(", ", words)` does the plain separator case without a stream.

## Your turn

- In `bracketed`, join the words with `", "` between them, `[` in front and `]` at the end.
