package oxide.maybe_there_is_none;

import java.util.Comparator;
import java.util.List;
import java.util.Optional;

public class Results {
    /** The player with the highest score, or empty when there are none. No ties. */
    public static Optional<Player> bestPlayer(List<Player> players) {
        return players.stream().max(Comparator.comparingInt(Player::score));
    }

    /** The best player's name, or "nobody". */
    public static String winnerName(List<Player> players) {
        return bestPlayer(players).map(Player::name).orElse("nobody");
    }
}
