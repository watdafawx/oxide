package oxide.count_with_merge;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class LoaderPage {
    /** Loader to how many mods use it. */
    public static Map<String, Integer> modsPerLoader(List<Mod> mods) {
        Map<String, Integer> counts = new HashMap<>();
        // TODO: for each mod, merge 1 into its loader's count with Integer::sum
        return counts;
    }

    /** Loader to the sum of its mods' downloads. */
    public static Map<String, Integer> downloadsPerLoader(List<Mod> mods) {
        Map<String, Integer> totals = new HashMap<>();
        // TODO: for each mod, merge its downloads into its loader's total with Integer::sum
        return totals;
    }
}
