package oxide.your_first_stream;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class ResultsTest {
    @Test
    void passingKeepsPlayersAtOrAboveMin() {
        List<Player> players = List.of(new Player("Ada", 70), new Player("Bob", 49), new Player("Grace", 50));
        assertThat(Results.passing(players, 50)).extracting(Player::name).containsExactly("Ada", "Grace");
        assertThat(players).hasSize(3);
    }

    @Test
    void longWordsKeepsWordsLongerThanN() {
        assertThat(Results.longWords(List.of("cat", "bookshelf", "planet", "umbrella"), 6))
                .containsExactly("bookshelf", "umbrella");
    }
}
