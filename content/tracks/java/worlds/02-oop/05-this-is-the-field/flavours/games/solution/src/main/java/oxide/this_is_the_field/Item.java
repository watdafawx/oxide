package oxide.this_is_the_field;

public class Item {
    private String id;
    private int maxStack;

    public Item(String id, int maxStack) {
        this.id = id;
        this.maxStack = maxStack;
    }

    public String id() {
        return id;
    }

    public int maxStack() {
        return maxStack;
    }
}
