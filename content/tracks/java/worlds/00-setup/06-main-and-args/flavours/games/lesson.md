**Why now:** the status bot needs to know *which* server to check. You will run it as `java StatusCommand play.example.net`, and the address arrives in `main` as an array of text. No address means check your own machine, `localhost`.

```java
public static void main(String[] args) {
    // java StatusCommand play.example.net  ->  args = {"play.example.net"}
    // java StatusCommand                   ->  args = {}
}
```

- `String[]` is an **array** of text. `args.length` is its size; `args[0]` is the first item. Counting starts at **0**.
- Asking for `args[0]` when the array is empty throws `ArrayIndexOutOfBoundsException`. Always check `args.length` first.
- `if (condition) { ... }` runs the block only when the condition is true.
- Real programs keep the logic in a normal method (here `header`) and let `main` just call it. That is what makes the logic testable.

## Your turn

- Open `src/main/java/oxide/main_and_args/StatusCommand.java`.
- Make `header(args)` return `Status of <first argument>`, or `Status of localhost` when there are no arguments.
- Leave `main` as it is; it already prints your result.
