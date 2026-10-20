package oxide.constructors_and_this;

public class Item {
    private final String name;
    private final int maxStack;

    public Item(String name, int maxStack) {
        // TODO: trim and lower-case the name; keep maxStack between 1 and 64
        this.name = name;
        this.maxStack = maxStack;
    }

    public Item(String name) {
        // TODO: call the two-argument constructor with this(...), stack size 64 (delete the two lines below it: this(...) must come first)
        this.name = name;
        this.maxStack = 0;
    }

    public Item() {
        // TODO: call another constructor with the name "air" (delete the two lines below it)
        this.name = "";
        this.maxStack = 0;
    }

    public String name() {
        return name;
    }

    public int maxStack() {
        return maxStack;
    }
}
