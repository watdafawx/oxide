package oxide.many_ways_to_build;

public class Item {
    private final String id;
    private final int maxStack;

    /** The full constructor: the rules live here. */
    public Item(String id, int maxStack) {
        this.id = id.trim().toLowerCase();
        this.maxStack = Math.max(1, Math.min(64, maxStack));
    }

    public Item(String id) {
        this(id, 64);
    }

    public Item() {
        this("air");
    }

    public String id() {
        return id;
    }

    public int maxStack() {
        return maxStack;
    }
}
