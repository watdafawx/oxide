package oxide.flatten_lists;

import java.util.List;

public class Packing {
    /** Every item of every order, in order. */
    public static List<String> allItems(List<List<String>> orders) {
        return orders.stream().flatMap(List::stream).toList();
    }
}
