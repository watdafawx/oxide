Two baskets hold apples, and you want the total. Before any maths, the code needs a way to hold a number under a name.

**Say it in Java.** You write `int small = 12;` and read it as "an int called small, set to 12". `int` is the type for a whole number. From that line on, the word `small` means 12.

**The word for it.** A name like this is a **variable**: a name for a value. The single `=` does not ask a question, it stores the value on the right into the name on the left.

**Maths with names.** Once a number has a name you can do maths with the name: `+` adds, `-` takes away, `*` multiplies. The answer can have a name too.

**Try it.** Three variables in a row, each declared once with its type:

```java
int boats = 4;
int crew = 3;
int people = boats * crew;   // 12
```

**Why bother.** You write each number once. Tomorrow a number changes: edit one line and everything that uses the name is right.

**Hand it back.** The last line of each method is `return` followed by the name or the sum. The method's `int` in front of its name says it hands back a whole number.

## Your turn

- In `totalApples`, make `int small` 12 and `int big` 3, then return their sum.
- In `roomLeft`, make `int capacity` 20 and `int apples` 12, then return `capacity - apples`.
- Save. Both objectives turn green.
