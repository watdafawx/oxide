package oxide.maps_key_to_value;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Shop {
    /** The price of item, or 0 when it is not listed. */
    public static int priceOf(Map<String, Integer> prices, String item) {
        return prices.getOrDefault(item, 0);
    }

    /** How often each word appears. */
    public static Map<String, Integer> countWords(List<String> words) {
        Map<String, Integer> counts = new HashMap<>();
        for (String w : words) {
            counts.put(w, counts.getOrDefault(w, 0) + 1);
        }
        return counts;
    }
}
