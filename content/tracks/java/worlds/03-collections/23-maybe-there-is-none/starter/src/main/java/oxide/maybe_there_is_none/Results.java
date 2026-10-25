package oxide.maybe_there_is_none;

import java.util.Comparator;
import java.util.List;
import java.util.Optional;

public class Results {
    /** The player with the highest score, or empty when there are none. No ties. */
    public static Optional<Player> bestPlayer(List<Player> players) {
        // TODO: stream the players and take the max by score
        return Optional.empty();
    }

    /** The best player's name, or "nobody". */
    public static String winnerName(List<Player> players) {
        // TODO: bestPlayer(players), map to the name, orElse "nobody"
        return "";
    }
}
