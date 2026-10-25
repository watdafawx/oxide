package oxide.maps_key_to_value;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Versions {
    /** The newest version for id, or "unknown" when it is not listed. */
    public static String latestVersion(Map<String, String> latest, String id) {
        return latest.getOrDefault(id, "unknown");
    }

    /** Loader to how many mods use it (one entry in loaders per mod). */
    public static Map<String, Integer> countPerLoader(List<String> loaders) {
        Map<String, Integer> counts = new HashMap<>();
        for (String l : loaders) {
            counts.put(l, counts.getOrDefault(l, 0) + 1);
        }
        return counts;
    }
}
