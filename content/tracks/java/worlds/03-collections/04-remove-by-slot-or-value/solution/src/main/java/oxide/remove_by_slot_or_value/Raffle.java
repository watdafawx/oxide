package oxide.remove_by_slot_or_value;

import java.util.ArrayList;
import java.util.List;

public class Raffle {
    /** A copy of the tickets without the one at position slot. */
    public static List<Integer> dropSlot(List<Integer> tickets, int slot) {
        List<Integer> copy = new ArrayList<>(tickets);
        copy.remove(slot);
        return copy;
    }

    /** A copy of the tickets without the ticket with this number. */
    public static List<Integer> dropNumber(List<Integer> tickets, int number) {
        List<Integer> copy = new ArrayList<>(tickets);
        copy.remove(Integer.valueOf(number));
        return copy;
    }
}
