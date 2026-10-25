package oxide.cut_a_line;

import java.util.List;

public class FeedCutter {
    /** "fabric,forge,quilt" to [fabric, forge, quilt]. */
    public static List<String> loaders(String line) {
        // TODO: split at commas and turn the array into a list
        return List.of();
    }

    /** "sodium 0.5.8 Fixed a crash in chunk loading" to "Fixed a crash in chunk loading". */
    public static String changelogOf(String line) {
        // TODO: split at spaces into at most 3 parts and return the last one
        return "";
    }
}
