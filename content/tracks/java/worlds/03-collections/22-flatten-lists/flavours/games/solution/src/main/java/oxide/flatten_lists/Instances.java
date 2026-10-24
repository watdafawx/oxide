package oxide.flatten_lists;

import java.util.List;

public class Instances {
    /** Every mod of every instance, in order. */
    public static List<Mod> allMods(List<List<Mod>> instances) {
        return instances.stream().flatMap(List::stream).toList();
    }
}
