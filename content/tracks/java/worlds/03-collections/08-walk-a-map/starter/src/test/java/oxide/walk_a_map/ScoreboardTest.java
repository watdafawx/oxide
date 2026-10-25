package oxide.walk_a_map;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.Map;
import org.junit.jupiter.api.Test;

class ScoreboardTest {
    private final Map<String, Integer> scores = Map.of("Owls", 41, "Foxes", 57, "Bears", 38, "Hares", -3);

    @Test
    void topScorerHasTheHighestScore() {
        assertThat(Scoreboard.topScorer(scores)).isEqualTo("Foxes");
        assertThat(Scoreboard.topScorer(Map.of("Solo", -5))).isEqualTo("Solo");
    }

    @Test
    void passingListsEveryNameAtOrAboveMin() {
        assertThat(Scoreboard.passing(scores, 41)).containsExactlyInAnyOrder("Owls", "Foxes");
        assertThat(Scoreboard.passing(scores, 100)).isEmpty();
    }
}
