package oxide.ask_for_a_list;

import java.util.List;

public class Words {
    /** The longest word; the first one wins a tie. The list is never empty. */
    public static String longestWord(List<String> words) {
        String best = words.get(0);
        for (String w : words) {
            if (w.length() > best.length()) {
                best = w;
            }
        }
        return best;
    }
}
