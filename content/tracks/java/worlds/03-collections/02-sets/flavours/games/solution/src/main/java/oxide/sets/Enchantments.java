package oxide.sets;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

public class Enchantments {
    /** The distinct lowercase enchantment ids of a space-separated line. */
    public static Set<String> uniqueEnchantments(String line) {
        return new HashSet<>(Arrays.asList(line.toLowerCase().split(" ")));
    }
}
