package oxide.split_in_two;

import static java.util.stream.Collectors.partitioningBy;

import java.util.List;
import java.util.Map;

public class UpdateScreen {
    /** true: mods not on the latest version for their id, false: the rest. Both keys always present. */
    public static Map<Boolean, List<Mod>> byOutdated(List<Mod> mods, Map<String, String> latest) {
        // TODO: collect with partitioningBy and a rule for "outdated"
        return Map.of();
    }
}
