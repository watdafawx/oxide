package oxide.count_with_merge;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Till {
    /** Item to how many sales it had. */
    public static Map<String, Integer> salesPerItem(List<Sale> sales) {
        Map<String, Integer> counts = new HashMap<>();
        for (Sale s : sales) {
            counts.merge(s.item(), 1, Integer::sum);
        }
        return counts;
    }

    /** Item to the sum of its sale amounts. */
    public static Map<String, Integer> takingsPerItem(List<Sale> sales) {
        Map<String, Integer> totals = new HashMap<>();
        for (Sale s : sales) {
            totals.merge(s.item(), s.amount(), Integer::sum);
        }
        return totals;
    }
}
