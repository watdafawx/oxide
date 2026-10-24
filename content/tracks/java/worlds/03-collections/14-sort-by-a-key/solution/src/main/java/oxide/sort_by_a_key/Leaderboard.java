package oxide.sort_by_a_key;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class Leaderboard {
    /** A new list: A to Z by name. */
    public static List<Player> byName(List<Player> players) {
        List<Player> copy = new ArrayList<>(players);
        copy.sort(Comparator.comparing(Player::name));
        return copy;
    }

    /** A new list: highest score first. */
    public static List<Player> byScoreDescending(List<Player> players) {
        List<Player> copy = new ArrayList<>(players);
        copy.sort(Comparator.comparingInt(Player::score).reversed());
        return copy;
    }
}
