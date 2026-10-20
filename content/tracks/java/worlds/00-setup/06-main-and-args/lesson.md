**Why this matters:** every Java program you run from a terminal starts in a method called `main`, and anything you type after the program name arrives as an array of text. Command-line tools, servers and bots all read their settings this way.

```java
public static void main(String[] args) {
    // java Banner Ada   ->  args = {"Ada"}
    // java Banner       ->  args = {}
}
```

- `String[]` is an **array** of text. `args.length` is its size; `args[0]` is the first item. Counting starts at **0**.
- Asking for `args[0]` when the array is empty throws `ArrayIndexOutOfBoundsException`. Always check `args.length` first.
- `+` glues text together: `"Hello, " + name + "!"` builds one `String`.
- `if (condition) { ... }` runs the block only when the condition is true. A `return` inside it ends the method early, so the code after the `if` only runs when the condition was false.
- Real programs keep the logic in a normal method (here `banner`) and let `main` just call it. That is what makes the logic testable.

## Your turn

- Open `src/main/java/oxide/main_and_args/Banner.java`.
- Make `banner(args)` return `Hello, <first argument>!`, or `Hello, Oxide!` when there are no arguments.
- Leave `main` as it is; it already prints your result.
