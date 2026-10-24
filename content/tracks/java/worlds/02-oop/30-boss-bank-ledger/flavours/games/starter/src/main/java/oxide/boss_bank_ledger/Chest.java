package oxide.boss_bank_ledger;

import java.util.Arrays;

public class Chest {
    private final String name;
    private String[] items = new String[0];
    private int[] counts = new int[0];
    private Event[] log = new Event[0];

    public Chest(String name) {
        this.name = name;
    }

    public String name() {
        return name;
    }

    /** How many of this item the chest holds. */
    public int count(String item) {
        for (int i = 0; i < items.length; i++) {
            if (items[i].equals(item)) {
                return counts[i];
            }
        }
        return 0;
    }

    /** How many items of any kind the chest holds. */
    public int total() {
        int sum = 0;
        for (int c : counts) {
            sum += c;
        }
        return sum;
    }

    /** A copy of the events applied so far, oldest first. */
    public Event[] log() {
        return log.clone();
    }

    /** Adds delta to the count of item (a negative delta takes away). Written for you: call it, do not change it. */
    private void change(String item, int delta) {
        for (int i = 0; i < items.length; i++) {
            if (items[i].equals(item)) {
                counts[i] += delta;
                return;
            }
        }
        items = Arrays.copyOf(items, items.length + 1);
        counts = Arrays.copyOf(counts, counts.length + 1);
        items[items.length - 1] = item;
        counts[counts.length - 1] = delta;
    }

    /** Adds one event to the end of the log. Written for you: call it, do not change it. */
    private void record(Event event) {
        log = Arrays.copyOf(log, log.length + 1);
        log[log.length - 1] = event;
    }

    /** Applies one event. A rejected one throws NotEnoughItemsException and changes nothing. */
    public void apply(Event event) {
        throw new UnsupportedOperationException("TODO: a switch statement over the three kinds of event, then record(event) once it has succeeded");
    }

    /** One numbered line per event, then "items: 40". Lines are separated by \n. */
    public String report() {
        throw new UnsupportedOperationException("TODO: build the report with a StringBuilder");
    }
}
