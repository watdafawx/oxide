package oxide.what_new_runs;

public class ItemStack {
    private String item;
    private int count;

    public ItemStack(String name, int amount) {
        item = name;
        if (amount > 64) {
            count = 64;
        } else {
            count = amount;
        }
    }

    public String item() {
        return item;
    }

    public int count() {
        return count;
    }
}
