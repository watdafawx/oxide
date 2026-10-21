A greeting that always says the same thing is no use. You want one that names whoever is arriving, so the method has to be handed a name and glue it into the text.

**Hand it a name.** A method can be given values to work with. They go in the brackets: `String name` means "I need a piece of text, and inside the method I will call it `name`". The word in the brackets is a **parameter**. The caller picks the actual text, for example `greet("Ada")`.

**Glue with plus.** A `+` between two pieces of text joins them into one. If `name` is `Ada`, then `"Hello, " + name` is `Hello, Ada`.

**Spaces are text too.** Nothing adds spaces for you. The space inside the quotes is part of the text, and if you forget it the words run together.

**Put it together.** The method takes the parameter and hands the glued text back:

```java
public static String greet(String name) {
    return "Hello, " + name + "!";
}
```

**More than one.** A method can take several parameters. Separate them with commas: `String first, String second`. Inside, each one is its own name.

**What the checker does.** It calls your method with different names, such as `greet("Ada")` and `greet("Bo")`, and compares the result with the text it expects ("Hello, Ada!").

## Your turn

- In `greet`, return the text with the name glued in.
- In `greetPair`, do the same with both names: "Hello, Ada and Bo!".
- Save. Both objectives turn green.
