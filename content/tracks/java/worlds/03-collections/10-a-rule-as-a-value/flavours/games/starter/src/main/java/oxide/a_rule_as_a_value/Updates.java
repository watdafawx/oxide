package oxide.a_rule_as_a_value;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.function.Predicate;

public class Updates {
    /** True for mods whose version is not the latest one listed for their id. */
    public static Predicate<Mod> isOutdated(Map<String, String> latest) {
        // TODO: return a lambda mod -> ... that is true when the version differs from latest.get(mod.id())
        return null;
    }

    /** A copy of mods without the outdated ones. */
    public static List<Mod> upToDate(List<Mod> mods, Map<String, String> latest) {
        List<Mod> copy = new ArrayList<>(mods);
        // TODO: remove every outdated mod from the copy with removeIf
        return copy;
    }
}
