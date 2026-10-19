package oxide.sets;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;

public class Sets {
    /** The distinct lowercase words of a space-separated text. */
    public static Set<String> uniqueWords(String text) {
        return new HashSet<>(Arrays.asList(text.toLowerCase().split(" ")));
    }
}
