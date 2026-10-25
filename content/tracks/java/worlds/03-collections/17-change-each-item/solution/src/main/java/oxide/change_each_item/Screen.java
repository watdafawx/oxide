package oxide.change_each_item;

import java.util.List;

public class Screen {
    /** Every player's name, in order. */
    public static List<String> names(List<Player> players) {
        return players.stream().map(Player::name).toList();
    }

    /** The names of the first three players (fewer if there are fewer). */
    public static List<String> podium(List<Player> ranked) {
        return ranked.stream().limit(3).map(Player::name).toList();
    }
}
