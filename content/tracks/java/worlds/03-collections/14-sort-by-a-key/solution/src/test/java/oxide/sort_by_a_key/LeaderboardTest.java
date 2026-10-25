package oxide.sort_by_a_key;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class LeaderboardTest {
    private final List<Player> players = List.of(new Player("Grace", 82), new Player("Ada", 70), new Player("Linus", 95));

    @Test
    void byNameSortsAtoZ() {
        assertThat(Leaderboard.byName(players)).extracting(Player::name).containsExactly("Ada", "Grace", "Linus");
        assertThat(players.get(0).name()).isEqualTo("Grace");
    }

    @Test
    void byScoreDescendingPutsTheBestFirst() {
        assertThat(Leaderboard.byScoreDescending(players)).extracting(Player::name).containsExactly("Linus", "Grace", "Ada");
    }
}
