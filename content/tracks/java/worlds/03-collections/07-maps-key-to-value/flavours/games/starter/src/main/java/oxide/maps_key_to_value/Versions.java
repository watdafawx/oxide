package oxide.maps_key_to_value;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Versions {
    /** The newest version for id, or "unknown" when it is not listed. */
    public static String latestVersion(Map<String, String> latest, String id) {
        // TODO: read the version with a fallback of "unknown"
        return null;
    }

    /** Loader to how many mods use it (one entry in loaders per mod). */
    public static Map<String, Integer> countPerLoader(List<String> loaders) {
        // TODO: for each loader, put its old count (0 if missing) plus one
        return null;
    }
}
