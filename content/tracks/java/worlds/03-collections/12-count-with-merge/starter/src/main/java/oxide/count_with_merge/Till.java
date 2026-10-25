package oxide.count_with_merge;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Till {
    /** Item to how many sales it had. */
    public static Map<String, Integer> salesPerItem(List<Sale> sales) {
        Map<String, Integer> counts = new HashMap<>();
        // TODO: for each sale, merge 1 into the item's count with Integer::sum
        return counts;
    }

    /** Item to the sum of its sale amounts. */
    public static Map<String, Integer> takingsPerItem(List<Sale> sales) {
        Map<String, Integer> totals = new HashMap<>();
        // TODO: for each sale, merge its amount into the item's total with Integer::sum
        return totals;
    }
}
