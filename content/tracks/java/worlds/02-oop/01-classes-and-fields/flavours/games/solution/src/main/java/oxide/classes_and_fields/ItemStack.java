package oxide.classes_and_fields;

public class ItemStack {
    private int count;

    /** How many items are in the stack. */
    public int count() {
        return count;
    }

    /** Adds one item to the stack. */
    public void grow() {
        count++;
    }
}
