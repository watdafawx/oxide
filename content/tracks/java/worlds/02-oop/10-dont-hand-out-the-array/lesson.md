A playlist keeps its songs in a private `String[]`, and `add` checks that no song appears twice. Then someone writes a getter, `songs()`, that hands the array out. Private fields, a careful `add`, and still the playlist can end up full of junk.

**Private guards the name.** `private` stops other code from writing `list.songs`. It does not stop the playlist's own methods from handing the array out, and a method that does `return songs;` does exactly that.

**Arrays are shared.** You saw this in world 01: passing an array hands over the very same array, not a copy. Returning one works the same way. So after `String[] mine = list.songs();`, the line `mine[0] = "junk";` changes the playlist itself, and `add` never ran.

**Hand out a copy.** `songs.clone()` builds a new array with the same values. Return that, and the caller can scribble on their copy all day:

```java
public String[] songs() {
    return songs.clone();      // the caller gets their own array
}
```

**What is safe already.** A getter that returns an `int` or a `long` hands back a copy of the number. A `String` cannot change at all (world 01). Arrays, and objects that have methods to change them, are the ones to copy.

**The rule of thumb.** Private fields, small public methods, and never hand out something the caller could change behind your back. That idea has a name: **encapsulation**, keeping an object's insides inside.

**For the curious.** World 03 brings lists, and with them `List.copyOf(...)` and read-only views, which solve the same leak without copying by hand.
