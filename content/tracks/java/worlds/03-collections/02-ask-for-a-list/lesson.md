Your last method handed back an `ArrayList`. A method that only reads a list, like "find the longest word", does not care how the list is stored. The starter asks for an `ArrayList` anyway, and this time the red is a compile error.

**Read the red.** The checker calls `longestWord` with a list made by `List.of(...)`, and the compiler refuses:

```text
WordsTest.java:13: error: incompatible types: List<String> cannot be converted to ArrayList<String>
        assertThat(Words.longestWord(fixed)).isEqualTo("spruce");
                                     ^
```

**More than one kind.** `ArrayList` is one way to store a list. Java has others: `LinkedList`, and the fixed lists that `List.of("a", "b")` makes. Every one of them can `get`, `size` and be looped over, but they are not `ArrayList`s.

**The promise.** `List` is an **interface**, like the ones in the OOP world: a promise of methods (`get`, `size`, `add`, ...) with no storage of its own. `ArrayList` is a class that keeps that promise, and so are the others. A `List<String>` parameter accepts any of them.

**Ask for the promise.** Ask for `ArrayList<String>` and you shut out every other kind of list, although your method would work on all of them. Ask for `List<String>` and they all come in.

**Left side too.** The same goes for variables: `List<String> words = new ArrayList<>();`. The left side says what you use, the right side picks how it is stored. Switch the right side later and no other line changes. Most Java code is written this way.

**Loop any list.** The enhanced `for` you used on arrays works on every `List` too: `for (String w : words)` visits each item in order.

```java
public static int letters(List<String> names) {   // any kind of list
    int total = 0;
    for (String n : names) {
        total += n.length();
    }
    return total;
}
```

## Your turn

- Change the parameter of `longestWord` from `ArrayList<String>` to `List<String>`. Save: the compile error is gone and the objective turns red instead.
- Return the longest word, keeping a champion as in the Basics world. Start from the first word; on a tie, keep the earlier one.
