package oxide.collectors;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;
import static java.util.stream.Collectors.joining;
import static java.util.stream.Collectors.partitioningBy;

import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class Report {
    /** Loader to the mods using it, in list order. */
    public static Map<String, List<Mod>> groupByLoader(List<Mod> mods) {
        throw new UnsupportedOperationException("TODO: group the mods by their loader");
    }

    /** Loader to how many mods use it. */
    public static Map<String, Long> countByLoader(List<Mod> mods) {
        throw new UnsupportedOperationException("TODO: group by loader again, but make each group a count instead of a list");
    }

    /** "[Sodium, Iris Shaders]", or "[]" for no mods. */
    public static String report(List<Mod> mods) {
        throw new UnsupportedOperationException("TODO: turn the mods into names, then join them with ', ' between, '[' in front and ']' at the end");
    }

    /** true: mods not on the latest version for their id, false: the rest. */
    public static Map<Boolean, List<Mod>> partitionByOutdated(List<Mod> mods, Map<String, String> latest) {
        throw new UnsupportedOperationException("TODO: split the mods into two groups: outdated (true) and up to date (false)");
    }
}
