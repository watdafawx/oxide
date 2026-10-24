package oxide.a_rule_as_a_value;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.function.Predicate;

public class Updates {
    /** True for mods whose version is not the latest one listed for their id. */
    public static Predicate<Mod> isOutdated(Map<String, String> latest) {
        return mod -> !mod.version().equals(latest.get(mod.id()));
    }

    /** A copy of mods without the outdated ones. */
    public static List<Mod> upToDate(List<Mod> mods, Map<String, String> latest) {
        List<Mod> copy = new ArrayList<>(mods);
        copy.removeIf(isOutdated(latest));
        return copy;
    }
}
