package oxide.stream_basics;

import java.util.List;
import java.util.Map;

public class Outdated {
    /** Names of the mods whose version is not the latest one listed for their id, in list order. */
    public static List<String> outdatedNames(List<Mod> mods, Map<String, String> latest) {
        return mods.stream()
                .filter(m -> !m.version().equals(latest.get(m.id())))
                .map(Mod::name)
                .toList();
    }

    /** Sum of the downloads of every mod. */
    public static int totalDownloads(List<Mod> mods) {
        return mods.stream().mapToInt(Mod::downloads).sum();
    }
}
