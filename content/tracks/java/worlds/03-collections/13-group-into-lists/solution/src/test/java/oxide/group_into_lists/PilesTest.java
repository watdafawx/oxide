package oxide.group_into_lists;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class PilesTest {
    @Test
    void byLengthGroupsWordsInOrder() {
        Map<Integer, List<String>> piles = Piles.byLength(List.of("cat", "horse", "dog", "mouse", "ox"));
        assertThat(piles).hasSize(3);
        assertThat(piles.get(3)).containsExactly("cat", "dog");
        assertThat(piles.get(5)).containsExactly("horse", "mouse");
        assertThat(piles.get(2)).containsExactly("ox");
    }
}
