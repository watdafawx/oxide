package oxide.update_report;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;

import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class UpdateReport {
    /** "sodium,Sodium,fabric,9500,0.5.3" into a Mod (id, name, loader, downloads, version). */
    public static Mod parseMod(String line) {
        throw new UnsupportedOperationException("TODO: split(\",\") and build a Mod; downloads is an int");
    }

    /** Loader to how many mods use it. */
    public static Map<String, Long> countsPerLoader(List<Mod> mods) {
        throw new UnsupportedOperationException("TODO: groupingBy loader with counting()");
    }

    /** Names of the n most downloaded mods: most first, ties A-Z. */
    public static List<String> topDownloaded(List<Mod> mods, int n) {
        throw new UnsupportedOperationException("TODO: sort by downloads descending then name, limit(n), map to names");
    }

    /** Loader to how many of its mods are outdated, loaders in A-Z order. */
    public static Map<String, Long> outdatedPerLoader(List<Mod> mods, Map<String, String> latest) {
        throw new UnsupportedOperationException("TODO: filter outdated mods, then groupingBy loader into a TreeMap");
    }
}
