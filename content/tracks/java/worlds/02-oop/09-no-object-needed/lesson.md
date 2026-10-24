The help desk wants one front door for new tickets: `Ticket.of("Fix login")`, which also tidies the title. There is no ticket yet when you ask for one, so this method cannot belong to a ticket. It belongs to the class.

**A static method.** A method marked **`static`** belongs to the class, like a static field. You call it through the class name, `Ticket.of("Fix login")`, with no object before the dot. Inside it there is no `this`, because no object is being worked on.

**Why your methods were static.** Every method in worlds 00 and 01 was `public static`, because there was no object to call it on: the checker called `Banner.banner(args)` straight on the class. `String.format(...)` is static for the same reason. Now you know both words: `public` lets anyone call it, `static` means no object needed.

**And main.** `public static void main(String[] args)` is static because when a program starts, no objects exist yet. Java needs one method it can call on the class alone, and that is `main`.

**One way in.** A static method that builds objects is a **factory**. Make the constructor `private` and the factory becomes the only way to get a ticket, so its tidy-up can never be skipped:

```java
public class Badge {
    private final String name;

    private Badge(String name) {          // only Badge's own code may call this
        this.name = name;
    }

    public static Badge issue(String name) {
        return new Badge(name.toUpperCase());
    }
}
```

## Your turn

- In `of`, build a `Ticket` from the trimmed title and hand it back.
- Make the constructor `private`.
- Save. Both objectives turn green.
