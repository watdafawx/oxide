An account's owner is set when it opens and never changes after that. Right now nothing says so: any method in the class could write `owner = "Mallory"` by accident. You can make the compiler hold that promise for you.

**Set once.** A field marked **`final`** gets its value in the constructor and can never be given another one. `private final String owner;` reads "owner is private, and set once".

**Try it on the wrong field.** Put `final` on `balance` as well and save. `deposit` changes the balance, so the compiler stops you:

```text
error: cannot assign a value to final variable balance
```

**Which fields.** That error is the rule: a field that some method must change cannot be final. Everything else should be. Most fields in good Java code are final, because a value that cannot change is one less thing to check when you hunt a bug.

**What it looks like.** A ticket keeps its seat for good but can be scanned later:

```java
public class Ticket {
    private final String seat;   // set in the constructor, then fixed
    private boolean scanned;     // changes, so not final
}
```

## Your turn

- Make the `owner` field `final`. Leave `balance` changeable.
- In `deposit`, add `cents` to the balance.
- Save. Both objectives turn green.
