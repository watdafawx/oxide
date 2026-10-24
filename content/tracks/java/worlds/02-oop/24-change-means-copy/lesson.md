The scales say the parcel lost 300 grams of packing. A record cannot change: its fields are final, there is no setter. So the answer is not to change this parcel, but to make a new one that differs in one value.

**Change means copy.** A method that "changes" a record builds a new record with one value swapped and hands it back. By habit it is named `with` plus the component: `withGrams`, `withLabel`. The original stays exactly as it was:

```java
public Tag withLevel(int newLevel) {
    return new Tag(name, newLevel);     // same name, new level
}
```

**Read the fields directly.** Inside the record, the components are plain names: `name`, `label`, `grams`. No accessor call needed.

**The rules come along.** The copy is built with `new`, so it passes through the compact constructor you wrote last quest. A negative weight is refused for the copy too, without a single extra line.

**Why bother.** Code that holds a parcel can trust it never shifts under its feet. Two parts of a program can share one record safely, because neither can change it on the other.

## Your turn

- In `withGrams`, build and hand back a new `Parcel` with the same label and `newGrams`.
- Save. Both objectives turn green.
