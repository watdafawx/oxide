package oxide.change_each_item;

import java.util.List;

public class Screen {
    /** Every player's name, in order. */
    public static List<String> names(List<Player> players) {
        // TODO: stream the players, map each to its name, toList()
        return List.of();
    }

    /** The names of the first three players (fewer if there are fewer). */
    public static List<String> podium(List<Player> ranked) {
        // TODO: the same pipeline, keeping only the first 3 players
        return List.of();
    }
}
