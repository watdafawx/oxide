package oxide.this_is_the_field;

public class Item {
    private String id;
    private int maxStack;

    public Item(String id, int maxStack) {
        // TODO: these two lines copy each parameter onto itself; make the left side the field
        id = id;
        maxStack = maxStack;
    }

    public String id() {
        return id;
    }

    public int maxStack() {
        return maxStack;
    }
}
