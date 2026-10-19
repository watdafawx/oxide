package oxide.sorting;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class Sorting {
    /** New list: shortest word first, ties A-Z. */
    public static List<String> byLengthThenAlpha(List<String> words) {
        List<String> copy = new ArrayList<>(words);
        copy.sort(Comparator.comparingInt(String::length).thenComparing(Comparator.naturalOrder()));
        return copy;
    }

    /** New list: highest score first. */
    public static List<Player> byScoreDescending(List<Player> players) {
        List<Player> copy = new ArrayList<>(players);
        copy.sort(Comparator.comparingInt(Player::score).reversed());
        return copy;
    }
}
