package oxide.your_first_stream;

import java.util.List;
import java.util.Map;

public class Filters {
    /** The mods not on the latest version listed for their id, in pack order. */
    public static List<Mod> outdated(List<Mod> mods, Map<String, String> latest) {
        return mods.stream().filter(m -> !m.version().equals(latest.get(m.id()))).toList();
    }

    /** The mods using this loader, in pack order. */
    public static List<Mod> onLoader(List<Mod> mods, String loader) {
        return mods.stream().filter(m -> m.loader().equals(loader)).toList();
    }
}
