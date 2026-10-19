package oxide.sorting;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class SortingTest {
    @Test
    void sortsByLengthThenAlpha() {
        List<String> input = List.of("pear", "fig", "banana", "apple", "kiwi", "date");
        assertThat(Sorting.byLengthThenAlpha(input)).containsExactly("fig", "date", "kiwi", "pear", "apple", "banana");
        assertThat(input).first().isEqualTo("pear");
    }

    @Test
    void sortsDescendingByScore() {
        List<Player> input = List.of(new Player("Ada", 70), new Player("Linus", 95), new Player("Grace", 82));
        assertThat(Sorting.byScoreDescending(input)).extracting(Player::name).containsExactly("Linus", "Grace", "Ada");
    }
}
