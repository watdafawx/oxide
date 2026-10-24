package oxide.walk_a_map;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class Popularity {
    /** The mod id with the most downloads. The map is never empty and has no ties. */
    public static String mostDownloaded(Map<String, Integer> downloads) {
        String best = null;
        int bestCount = Integer.MIN_VALUE;
        for (Map.Entry<String, Integer> e : downloads.entrySet()) {
            if (e.getValue() > bestCount) {
                best = e.getKey();
                bestCount = e.getValue();
            }
        }
        return best;
    }

    /** Every mod id with at least min downloads, in any order. */
    public static List<String> popular(Map<String, Integer> downloads, int min) {
        List<String> out = new ArrayList<>();
        for (Map.Entry<String, Integer> e : downloads.entrySet()) {
            if (e.getValue() >= min) {
                out.add(e.getKey());
            }
        }
        return out;
    }
}
