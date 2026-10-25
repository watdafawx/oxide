package oxide.an_object_remembers;

public class Counter {
    int count;

    /** The current count. */
    public int value() {
        return count;
    }

    /** Adds one to the count. */
    public void increment() {
        count++;
    }
}
