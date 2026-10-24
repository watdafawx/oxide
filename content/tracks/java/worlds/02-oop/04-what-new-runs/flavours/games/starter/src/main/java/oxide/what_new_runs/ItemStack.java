package oxide.what_new_runs;

public class ItemStack {
    private String item;
    private int count;

    public ItemStack(String name, int amount) {
        // TODO: store name in item, and amount in count (but 64 when amount is above 64)
    }

    public String item() {
        return item;
    }

    public int count() {
        return count;
    }
}
