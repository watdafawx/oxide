The boss's `render` needs two things that do not mix: it reads the bar's readings, and it hands `cx` to `widget_header` to make the drag listener. Rust refuses that order unless the reading ends first. This page shows why, and the small block that fixes it.

**Reading an entity.** `self.bar.read(cx)` gives a `&StatsBar`: a borrow of the bar, which lives in the app's store of entities. You reach that store through `cx`, so for as long as you use `bar`, `cx` counts as borrowed for reading too.

**Then you need cx to change things.** `widget_header(cx)` takes `&mut Context<WidgetView>`, because making a listener changes the app. A `&mut` borrow must be the only one, and `bar` is still around. Rust stops the build:

```text
error[E0502]: cannot borrow `*cx` as mutable because it is also borrowed as immutable
```

**Why Rust cares.** Through a `&mut` app you could change the bar, while `bar` still points at its old readings. Reading half-changed data is exactly what the one-writer rule from [One writer at a time](/quest/rust/02-ownership/08-one-writer-at-a-time) prevents.

**A block ends the borrow.** Put everything that needs `bar` inside `{ }`, and make the block's last line the finished elements, with no semicolon, so the block gives them back: `let content = { let bar = self.bar.read(cx); .. groups };`. At the closing brace, `bar` and any closure that used it are gone, so the borrow is over.

**The elements live on.** The rows and headings built inside keep their own copies of the texts and numbers; they do not point into the bar. So they outlive the block, and after it you can call `widget_header(cx)` and add `content` to the outer `div` with `.children(content)`.

**For the curious.** Rust actually ends a borrow after its last use, not at a brace. The block makes that point visible and hard to break: a later edit cannot use `bar` after `cx` was lent out by accident.
