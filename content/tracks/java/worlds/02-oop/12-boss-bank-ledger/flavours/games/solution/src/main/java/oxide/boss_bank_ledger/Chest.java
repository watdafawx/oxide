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
        switch (event) {
            case ItemPlaced p -> items.merge(p.stack().item(), p.stack().count(), Integer::sum);
            case ItemTaken t -> take(t.stack());
            case ItemMoved m -> {
                take(m.stack());
                m.to().apply(new ItemPlaced(m.stack()));
            }
        }
        log.add(event);
    }

    private void take(ItemStack stack) {
        if (count(stack.item()) < stack.count()) {
            throw new NotEnoughItemsException(
                    name + " holds " + count(stack.item()) + " x " + stack.item() + ", not " + stack.count());
        }
        items.merge(stack.item(), -stack.count(), Integer::sum);
    }

    /** One numbered line per event, then "items: 40". Lines are joined with \n. */
    public String report() {
        List<String> lines = new ArrayList<>();
        int number = 1;
        for (Event event : log) {
            String text = switch (event) {
                case ItemPlaced p -> "placed " + p.stack().count() + " x " + p.stack().item();
                case ItemTaken t -> "took " + t.stack().count() + " x " + t.stack().item();
                case ItemMoved m -> "moved " + m.stack().count() + " x " + m.stack().item() + " to " + m.to().name();
            };
            lines.add(number + ". " + text);
            number++;
        }
        lines.add("items: " + total());
        return String.join("\n", lines);
    }
}
