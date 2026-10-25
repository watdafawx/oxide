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
        String[] parts = line.split(",");
        return new Mod(parts[0], parts[1], parts[2], Integer.parseInt(parts[3]), parts[4]);
    }

    /** Loader to how many mods use it. */
    public static Map<String, Long> countsPerLoader(List<Mod> mods) {
        return mods.stream().collect(groupingBy(Mod::loader, counting()));
    }

    /** Names of the n most downloaded mods: most first, ties A to Z. */
    public static List<String> topDownloaded(List<Mod> mods, int n) {
        List<Mod> ranked = new ArrayList<>(mods);
        ranked.sort(Comparator.comparingInt(Mod::downloads).reversed().thenComparing(Mod::name));
        return ranked.stream().limit(n).map(Mod::name).toList();
    }

    /** Loader to how many of its mods are outdated, loaders in A to Z order. */
    public static Map<String, Long> outdatedPerLoader(List<Mod> mods, Map<String, String> latest) {
        return new TreeMap<>(mods.stream()
                .filter(m -> !m.version().equals(latest.get(m.id())))
                .collect(groupingBy(Mod::loader, counting())));
    }
}
