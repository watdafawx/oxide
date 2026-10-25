Scores are whole numbers, so the obvious list is `List<int>`. Try it, and the compiler says no. Today you meet the reason, and the one-word fix.

**Try the obvious thing.** `List<int> scores = new ArrayList<>();` stops at the first word:

```text
Scores.java:7: error: unexpected type
    List<int> scores = new ArrayList<>();
         ^
  required: reference
  found:    int
```

**Values and objects.** `int`, `long`, `double`, `boolean` and `char` are **primitive** types: bare values, stored as they are. Generics only work with objects, which the message calls a "reference". Java's lists held objects for years before generics arrived, and generics were built on top of that, so primitives were left out.

**The box.** For each primitive, Java has a class that wraps one value: `Integer` for `int`, `Long` for `long`, `Double` for `double`, `Character` for `char`, `Boolean` for `boolean`. So a list of whole numbers is a `List<Integer>`.

**Java boxes for you.** `scores.add(42)` wraps the `int` in an `Integer` on the way in (**boxing**), and `int first = scores.get(0);` takes it back out (**unboxing**). You keep writing ordinary numbers and ordinary `+`.

```java
List<Integer> pings = new ArrayList<>();
pings.add(42);                // int in, Integer stored
pings.add(87);
int worst = pings.get(1);     // Integer out, an int again
for (int p : pings) { }       // each one unboxed as you loop
```

**Two traps.** A box can be empty: `null`. Unboxing `null` into an `int` stops the program with a `NullPointerException`. And compare two `Integer`s with `equals`, not `==`: like text, `==` asks "same box?", not "same number?".

**For the curious.** Java reuses boxes for -128 to 127, so `==` on two `Integer`s seems to work in small tests and fails from 128 up. Boxing also costs memory: a million `Integer`s are a million small objects, which is why the streams later in this world have special number versions.

## Your turn

- In `total`, add up every score: an `int` running total and an enhanced `for` over the list.
- In `curved`, build a new `List<Integer>` with `bonus` added to every score, in the same order.
