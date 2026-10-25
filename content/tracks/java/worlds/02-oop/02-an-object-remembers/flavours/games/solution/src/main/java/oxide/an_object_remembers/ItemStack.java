package oxide.an_object_remembers;

public class ItemStack {
    int count;

    /** How many items are in the stack. */
    public int count() {
        return count;
    }

    /** Adds one item to the stack. */
    public void grow() {
        count++;
    }
}
