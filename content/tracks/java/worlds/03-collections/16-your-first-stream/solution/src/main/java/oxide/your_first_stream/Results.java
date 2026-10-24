package oxide.your_first_stream;

import java.util.List;

public class Results {
    /** The players with a score of at least min, in input order. */
    public static List<Player> passing(List<Player> players, int min) {
        return players.stream().filter(p -> p.score() >= min).toList();
    }

    /** The words longer than n letters, in input order. */
    public static List<String> longWords(List<String> words, int n) {
        return words.stream().filter(w -> w.length() > n).toList();
    }
}
