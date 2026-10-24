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
        // TODO: replace these two lines with one this(...) call: the full constructor, stack size 64
        this.id = id;
        this.maxStack = 0;
    }

    public Item() {
        // TODO: replace these two lines with one this(...) call, with the id "air"
        this.id = "";
        this.maxStack = 0;
    }

    public String id() {
        return id;
    }

    public int maxStack() {
        return maxStack;
    }
}
