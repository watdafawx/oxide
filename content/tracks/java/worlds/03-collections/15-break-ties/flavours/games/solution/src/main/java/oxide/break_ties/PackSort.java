package oxide.break_ties;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class PackSort {
    /** A new list: by loader A to Z, then by name A to Z inside each loader. */
    public static List<Mod> byLoaderThenName(List<Mod> mods) {
        List<Mod> copy = new ArrayList<>(mods);
        copy.sort(Comparator.comparing(Mod::loader).thenComparing(Mod::name));
        return copy;
    }
}
