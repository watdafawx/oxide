A zombie is a zombie from spawn to death; it never turns into a cow. Right now nothing says so: any method in the class could write `type = "cow"` by accident. You can make the compiler hold that promise for you.

**Set once.** A field marked **`final`** gets its value in the constructor and can never be given another one. `private final String type;` reads "type is private, and set once".

**Try it on the wrong field.** Put `final` on `health` as well and save. `hurt` changes the health, so the compiler stops you:

```text
error: cannot assign a value to final variable health
```

**Which fields.** That error is the rule: a field that some method must change cannot be final. Everything else should be. Most fields in good Java code are final, because a value that cannot change is one less thing to check when you hunt a bug.

**What it looks like.** A sign keeps its position for good, but its text can be edited:

```java
public class Sign {
    private final int x;         // set in the constructor, then fixed
    private String text;         // changes, so not final
}
```

## Your turn

- Make the `type` field `final`. Leave `health` changeable.
- In `hurt`, take `amount` away from the health.
- Save. Both objectives turn green.
