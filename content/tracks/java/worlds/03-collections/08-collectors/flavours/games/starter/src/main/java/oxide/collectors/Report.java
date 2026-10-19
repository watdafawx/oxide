package oxide.collectors;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;
import static java.util.stream.Collectors.joining;
import static java.util.stream.Collectors.partitioningBy;

import java.util.List;
import java.util.Map;

public class Report {
    /** Loader to the mods using it, in list order. */
    public static Map<String, List<Mod>> groupByLoader(List<Mod> mods) {
        throw new UnsupportedOperationException("TODO: collect(groupingBy(Mod::loader))");
    }

    /** Loader to how many mods use it. */
    public static Map<String, Long> countByLoader(List<Mod> mods) {
        throw new UnsupportedOperationException("TODO: groupingBy with a counting() downstream collector");
    }

    /** "[Sodium, Iris Shaders]", or "[]" for no mods. */
    public static String report(List<Mod> mods) {
        throw new UnsupportedOperationException("TODO: map to names, then collect(joining(separator, prefix, suffix))");
    }

    /** true: mods not on the latest version for their id, false: the rest. */
    public static Map<Boolean, List<Mod>> partitionByOutdated(List<Mod> mods, Map<String, String> latest) {
        throw new UnsupportedOperationException("TODO: collect(partitioningBy(...))");
    }
}
