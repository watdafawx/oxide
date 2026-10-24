package oxide.break_ties;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class Leaderboard {
    /** A new list: highest score first, equal scores A to Z by name. */
    public static List<Player> byScoreThenName(List<Player> players) {
        List<Player> copy = new ArrayList<>(players);
        // TODO: add the name as a tie-break after the reversed score
        copy.sort(Comparator.comparingInt(Player::score).reversed());
        return copy;
    }
}
