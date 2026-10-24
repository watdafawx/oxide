package oxide.private_means_mine;

public class ItemStack {
    private int count;

    /** Adds items to the stack. */
    public void grow(int amount) {
        count = count + amount;
    }

    /** How many items are in the stack. */
    public int count() {
        return count;
    }

    /** Removes items when the stack has enough. Hands back whether it did. */
    public boolean shrink(int amount) {
        if (amount > count) {
            return false;
        }
        count = count - amount;
        return true;
    }
}
