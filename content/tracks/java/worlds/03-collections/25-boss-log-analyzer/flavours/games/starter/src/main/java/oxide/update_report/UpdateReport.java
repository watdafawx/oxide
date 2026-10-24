package oxide.update_report;

import static java.util.stream.Collectors.counting;
import static java.util.stream.Collectors.groupingBy;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class UpdateReport {
    /** "sodium,Sodium,fabric,9500,0.5.3" into a Mod (id, name, loader, downloads, version). */
    public static Mod parseMod(String line) {
        // TODO: split at commas into 5 parts and build a Mod; Integer.parseInt(parts[3]) turns the downloads into an int
        return null;
    }

    /** Loader to how many mods use it. */
    public static Map<String, Long> countsPerLoader(List<Mod> mods) {
        // TODO: group the mods by loader and count each group
        return Map.of();
    }

    /** Names of the n most downloaded mods: most first, ties A to Z. */
    public static List<String> topDownloaded(List<Mod> mods, int n) {
        // TODO: sort a copy by downloads (most first) then name, keep the first n names
        return List.of();
    }

    /** Loader to how many of its mods are outdated, loaders in A to Z order. */
    public static Map<String, Long> outdatedPerLoader(List<Mod> mods, Map<String, String> latest) {
        // TODO: keep the outdated mods, count them per loader,
        // and return the counts copied into a new TreeMap so the loaders are sorted
        return Map.of();
    }
}
