The bot prints three kinds of things: a player count, an online flag and a MOTD. It would be nice to call all three `show` and let Java work out which one is meant.

**One name, many jobs.** Java lets several methods share a name, as long as their parameters are different. That is called **overloading**.

**Java picks by input.** When you call the name, Java looks at how many arguments you passed and what types they have, and runs the method that fits. `label(3)` and `label(true)` call different methods:

**Two methods, one name.** Same name, different parameter type, so Java can tell them apart:

```java
public static String label(int n)      { return String.format("%d items", n); }
public static String label(boolean on) { return on ? "on" : "off"; }
```

**What must differ.** The parameter list: a different number of parameters, or different types. The return type alone cannot tell two methods apart, because the call does not say what you will do with the result.

**You already use it.** `System.out.println` works for text, numbers and booleans because it is overloaded in exactly this way.

## Your turn

- In `show(int)`, return the count followed by `online`, like `5 online`.
- In `show(boolean)`, return `ONLINE` or `OFFLINE`.
- In `show(String)`, return `MOTD: ` followed by the text. Save and the objective turns green.
