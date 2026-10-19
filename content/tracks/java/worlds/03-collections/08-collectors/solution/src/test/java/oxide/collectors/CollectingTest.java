package oxide.collectors;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class CollectingTest {
    @Test
    void groupsByLength() {
        List<String> words = List.of("cat", "horse", "dog", "mouse", "ox");
        assertThat(Collecting.groupByLength(words))
                .hasSize(3)
                .containsEntry(3, List.of("cat", "dog"))
                .containsEntry(5, List.of("horse", "mouse"))
                .containsEntry(2, List.of("ox"));
        assertThat(Collecting.countByLength(words))
                .containsEntry(3, 2L)
                .containsEntry(5, 2L)
                .containsEntry(2, 1L);
    }

    @Test
    void joinsWithCommas() {
        assertThat(Collecting.joinWithCommas(List.of("a", "b", "c"))).isEqualTo("[a, b, c]");
        assertThat(Collecting.joinWithCommas(List.of("solo"))).isEqualTo("[solo]");
        assertThat(Collecting.joinWithCommas(List.of())).isEqualTo("[]");
    }

    @Test
    void partitionsPassFail() {
        var parts = Collecting.partitionPassFail(List.of(80, 49, 50, 12, 99));
        assertThat(parts.get(true)).containsExactly(80, 50, 99);
        assertThat(parts.get(false)).containsExactly(49, 12);
        assertThat(Collecting.partitionPassFail(List.of()).get(true)).isEmpty();
    }
}
