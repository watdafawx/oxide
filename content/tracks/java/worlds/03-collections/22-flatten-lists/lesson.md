A shop's orders are a `List<List<String>>`: each order is a list of items. The packing station wants one list of every item to fetch, across all orders. That needs one step that the streams so far did not have.

**Lists of lists.** `List<List<String>>` is a list whose items are lists, read from the outside in: "a list of lists of String". Streaming it gives one item per order, not per thing to fetch.

**map is not enough.** `map(order -> order.stream())` turns each order into a stream, so you get a stream of streams, and `toList()` would hand back a list of streams. Not what the packers want.

**flatMap.** `.flatMap(fn)` also turns each item into a stream, then glues all those streams end to end into one. One order of 2 items and one of 3 give a single stream of 5 items, in order. An empty order adds nothing.

```java
List<List<Integer>> rounds = List.of(List.of(3, 5), List.of(), List.of(8));
List<Integer> all = rounds.stream()
        .flatMap(List::stream)
        .toList();                 // [3, 5, 8]
```

**For the curious.** `flatMap` is also the tool for "one item becomes zero, one or many": for example a line of text becomes its words. Streams of other types have their own version, like `flatMapToInt`.

## Your turn

- In `allItems`, turn the list of orders into one list of every item, in order, with `flatMap`.
