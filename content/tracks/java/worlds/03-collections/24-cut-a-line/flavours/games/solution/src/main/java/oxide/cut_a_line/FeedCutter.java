package oxide.cut_a_line;

import java.util.List;

public class FeedCutter {
    /** "fabric,forge,quilt" to [fabric, forge, quilt]. */
    public static List<String> loaders(String line) {
        return List.of(line.split(","));
    }

    /** "sodium 0.5.8 Fixed a crash in chunk loading" to "Fixed a crash in chunk loading". */
    public static String changelogOf(String line) {
        return line.split(" ", 3)[2];
    }
}
