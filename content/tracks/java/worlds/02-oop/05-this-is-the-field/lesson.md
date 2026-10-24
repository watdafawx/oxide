Last quest the constructor's parameters had other names than the fields: `name` went into `owner`. Real Java code nearly always gives them the same name, because `owner` is the right word for both. That is where a quiet bug lives.

**Try the obvious thing.** The starter's constructor says `owner = owner;` and `balance = balance;`. It compiles without a word. Save and read the red: the owner is `null` and the balance is `0`, as if the constructor did nothing.

**What went wrong.** Inside the constructor there are two things called `owner`: the field and the parameter. A plain name always means the nearest one, the parameter. So `owner = owner;` copies the parameter onto itself, and the field is never touched.

**The word for it.** **`this`** means "the object these steps are working on", here the account being built. `this.owner` is that account's field, so `this.owner = owner;` reads "my field gets the parameter":

```java
public Badge(String holder, int level) {
    this.holder = holder;      // field = parameter
    this.level = level;
}
```

**Why the same name.** You could keep inventing other names, but `owner`, `name` and `theOwner` for one idea is how readers get lost. Same name plus `this.` is the habit you will see in every Java codebase.

## Your turn

- In the constructor, put `this.` in front of the field on the left of both lines.
- Save. Both objectives turn green.
