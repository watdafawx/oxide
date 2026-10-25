package oxide.cut_a_line;

import java.util.List;

public class LineCutter {
    /** The words of a sentence with single spaces between them. */
    public static List<String> words(String text) {
        return List.of(text.split(" "));
    }

    /** "10:15:30 ERROR db Connection refused" to "Connection refused". */
    public static String messageOf(String line) {
        return line.split(" ", 4)[3];
    }
}
