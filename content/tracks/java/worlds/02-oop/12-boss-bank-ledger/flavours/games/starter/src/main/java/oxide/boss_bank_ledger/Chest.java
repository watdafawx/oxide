package oxide.boss_bank_ledger;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Chest {
    private final String name;
    private final Map<String, Integer> items = new HashMap<>();
    private final List<Event> log = new ArrayList<>();

    public Chest(String name) {
        this.name = name;
    }

    public String name() {
        return name;
    }

    /** How many of this item the chest holds. */
    public int count(String item) {
        return items.getOrDefault(item, 0);
    }

    /** How many items of any kind the chest holds. */
    public int total() {
        int sum = 0;
        for (int count : items.values()) {
            sum += count;
        }
        return sum;
    }

    /** A copy of the events applied so far, oldest first. */
    public List<Event> log() {
        return List.copyOf(log);
    }

    /** Applies one event. A rejected one throws and changes nothing. */
    public void apply(Event event) {
        // TODO: switch (event) over ItemPlaced, ItemTaken and ItemMoved; add event to log once it has succeeded
        throw new UnsupportedOperationException("TODO: apply the event");
    }

    /** One numbered line per event, then "items: 40". Lines are joined with \n. */
    public String report() {
        throw new UnsupportedOperationException("TODO: build the report");
    }
}
