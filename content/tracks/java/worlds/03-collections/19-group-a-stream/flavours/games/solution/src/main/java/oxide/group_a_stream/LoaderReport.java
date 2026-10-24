package oxide.group_a_stream;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;

import java.util.List;
import java.util.Map;

public class LoaderReport {
    /** Loader to the mods using it, in pack order. */
    public static Map<String, List<Mod>> groupByLoader(List<Mod> mods) {
        return mods.stream().collect(groupingBy(Mod::loader));
    }

    /** Loader to how many mods use it. */
    public static Map<String, Long> countByLoader(List<Mod> mods) {
        return mods.stream().collect(groupingBy(Mod::loader, counting()));
    }
}
