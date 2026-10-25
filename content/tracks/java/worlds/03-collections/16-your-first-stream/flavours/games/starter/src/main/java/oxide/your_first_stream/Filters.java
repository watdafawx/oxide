package oxide.your_first_stream;

import java.util.List;
import java.util.Map;

public class Filters {
    /** The mods not on the latest version listed for their id, in pack order. */
    public static List<Mod> outdated(List<Mod> mods, Map<String, String> latest) {
        // TODO: stream the mods, filter on the version, end with toList()
        return List.of();
    }

    /** The mods using this loader, in pack order. */
    public static List<Mod> onLoader(List<Mod> mods, String loader) {
        // TODO: stream the mods, filter on the loader, end with toList()
        return List.of();
    }
}
