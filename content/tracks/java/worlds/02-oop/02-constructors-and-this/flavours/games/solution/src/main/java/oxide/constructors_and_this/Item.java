package oxide.constructors_and_this;

public class Item {
    private final String name;
    private final int maxStack;

    public Item(String name, int maxStack) {
        this.name = name.trim().toLowerCase();
        this.maxStack = Math.max(1, Math.min(64, maxStack));
    }

    public Item(String name) {
        this(name, 64);
    }

    public Item() {
        this("air");
    }

    public String name() {
        return name;
    }

    public int maxStack() {
        return maxStack;
    }
}
