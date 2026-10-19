package oxide.lists;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class ListsTest {
    @Test
    void removesByIndexAndValue() {
        assertThat(Lists.removeIndexAndValue(List.of(10, 20, 30, 40), 0, 30)).containsExactly(20, 40);
        // value 3 must not be treated as index 3
        assertThat(Lists.removeIndexAndValue(List.of(5, 6, 7, 3), 0, 3)).containsExactly(6, 7);
    }

    @Test
    void listGrowsPastInitialSize() {
        List<String> labels = Lists.labels(100);
        assertThat(labels).hasSize(100);
        assertThat(labels.get(0)).isEqualTo("item0");
        assertThat(labels.get(99)).isEqualTo("item99");
    }
}
