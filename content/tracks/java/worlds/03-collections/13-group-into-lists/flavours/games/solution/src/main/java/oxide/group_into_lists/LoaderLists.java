package oxide.group_into_lists;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class LoaderLists {
    /** Loader to the names of its mods, in pack order. */
    public static Map<String, List<String>> namesByLoader(List<Mod> mods) {
        Map<String, List<String>> groups = new HashMap<>();
        for (Mod mod : mods) {
            groups.computeIfAbsent(mod.loader(), k -> new ArrayList<>()).add(mod.name());
        }
        return groups;
    }
}
