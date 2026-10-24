An inventory keeps its item ids in a private `String[] slots`, and `put` checks that only real items go in. Then someone writes a getter, `slots()`, that hands the array out. Private fields, a careful `put`, and still a player can end up with a free diamond.

**Private guards the name.** `private` stops other code from writing `inv.slots`. It does not stop the inventory's own methods from handing the array out, and a method that does `return slots;` does exactly that.

**Arrays are shared.** You saw this in world 01: passing an array hands over the very same array, not a copy. Returning one works the same way. So after `String[] mine = inv.slots();`, the line `mine[0] = "diamond";` changes the inventory itself, and `put` never ran. In a mod, that is a dupe bug.

**Hand out a copy.** `slots.clone()` builds a new array with the same values. Return that, and the caller can scribble on their copy all day:

```java
public String[] slots() {
    return slots.clone();      // the caller gets their own array
}
```

**What is safe already.** A getter that returns an `int` or a `long` hands back a copy of the number. A `String` cannot change at all (world 01). Arrays, and objects that have methods to change them, are the ones to copy.

**The rule of thumb.** Private fields, small public methods, and never hand out something the caller could change behind your back. That idea has a name: **encapsulation**, keeping an object's insides inside.

**For the curious.** World 03 brings lists, and with them `List.copyOf(...)` and read-only views, which solve the same leak without copying by hand.
