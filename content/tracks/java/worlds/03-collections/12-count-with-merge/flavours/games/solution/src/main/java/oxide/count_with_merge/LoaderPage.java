package oxide.count_with_merge;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class LoaderPage {
    /** Loader to how many mods use it. */
    public static Map<String, Integer> modsPerLoader(List<Mod> mods) {
        Map<String, Integer> counts = new HashMap<>();
        for (Mod mod : mods) {
            counts.merge(mod.loader(), 1, Integer::sum);
        }
        return counts;
    }

    /** Loader to the sum of its mods' downloads. */
    public static Map<String, Integer> downloadsPerLoader(List<Mod> mods) {
        Map<String, Integer> totals = new HashMap<>();
        for (Mod mod : mods) {
            totals.merge(mod.loader(), mod.downloads(), Integer::sum);
        }
        return totals;
    }
}
