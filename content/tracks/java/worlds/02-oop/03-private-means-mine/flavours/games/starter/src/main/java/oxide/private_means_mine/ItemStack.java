package oxide.private_means_mine;

public class ItemStack {
    // TODO: make this field private, so only ItemStack's own methods can change it
    int count;

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
        throw new UnsupportedOperationException("TODO: refuse when amount is more than count, otherwise remove them");
    }
}
