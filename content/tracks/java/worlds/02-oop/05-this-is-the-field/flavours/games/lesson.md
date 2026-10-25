Last quest the constructor's parameters had other names than the fields: `name` went into `item`. Real mod code nearly always gives them the same name, because `id` is the right word for both. That is where a quiet bug lives.

**Try the obvious thing.** The starter's constructor says `id = id;` and `maxStack = maxStack;`. It compiles without a word. Save and read the red: the id is `null` and the stack size is `0`, as if the constructor did nothing.

**What went wrong.** Inside the constructor there are two things called `id`: the field and the parameter. A plain name always means the nearest one, the parameter. So `id = id;` copies the parameter onto itself, and the field is never touched.

**The word for it.** **`this`** means "the object these steps are working on", here the item being built. `this.id` is that item's field, so `this.id = id;` reads "my field gets the parameter":

```java
public Enchantment(String name, int level) {
    this.name = name;          // field = parameter
    this.level = level;
}
```

**Why the same name.** You could keep inventing other names, but `id`, `name` and `theId` for one idea is how readers get lost. Same name plus `this.` is the habit you will see in every mod's source.

## Your turn

- In the constructor, put `this.` in front of the field on the left of both lines.
- Save. Both objectives turn green.
