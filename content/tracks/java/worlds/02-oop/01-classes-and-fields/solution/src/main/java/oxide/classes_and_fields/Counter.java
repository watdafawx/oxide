package oxide.classes_and_fields;

public class Counter {
    private int count;

    /** The current count. */
    public int value() {
        return count;
    }

    /** Adds one to the count. */
    public void increment() {
        count++;
    }
}
