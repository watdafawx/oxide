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
        String[] parts = line.split(",");
        return new Mod(parts[0], parts[1], parts[2], Integer.parseInt(parts[3]), parts[4]);
    }

    /** Loader to how many mods use it. */
    public static Map<String, Long> countsPerLoader(List<Mod> mods) {
        return mods.stream().collect(groupingBy(Mod::loader, counting()));
    }

    /** Names of the n most downloaded mods: most first, ties A-Z. */
    public static List<String> topDownloaded(List<Mod> mods, int n) {
        return mods.stream()
                .sorted(Comparator.comparingInt(Mod::downloads).reversed().thenComparing(Mod::name))
                .limit(n)
                .map(Mod::name)
                .toList();
    }

    /** Loader to how many of its mods are outdated, loaders in A-Z order. */
    public static Map<String, Long> outdatedPerLoader(List<Mod> mods, Map<String, String> latest) {
        return mods.stream()
                .filter(m -> !m.version().equals(latest.get(m.id())))
                .collect(groupingBy(Mod::loader, TreeMap::new, counting()));
    }
}
