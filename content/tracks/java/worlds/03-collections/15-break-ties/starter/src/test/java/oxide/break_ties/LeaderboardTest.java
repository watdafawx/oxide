package oxide.break_ties;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class LeaderboardTest {
    @Test
    void byScoreThenNameBreaksTiesAtoZ() {
        List<Player> players = List.of(
                new Player("Grace", 82), new Player("Linus", 95), new Player("Ada", 82), new Player("Bob", 70));
        assertThat(Leaderboard.byScoreThenName(players)).extracting(Player::name)
                .containsExactly("Linus", "Ada", "Grace", "Bob");
    }
}
