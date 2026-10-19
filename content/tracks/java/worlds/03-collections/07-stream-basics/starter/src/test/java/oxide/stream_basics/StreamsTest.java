package oxide.stream_basics;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class StreamsTest {
    @Test
    void filtersEvenSquares() {
        assertThat(Streams.evenSquares(List.of(1, 2, 3, 4, 5, 6))).containsExactly(4, 16, 36);
        assertThat(Streams.evenSquares(List.of(1, 3, 5))).isEmpty();
    }

    @Test
    void sumsWithMapToInt() {
        assertThat(Streams.totalLength(List.of("ab", "cde", ""))).isEqualTo(5);
        assertThat(Streams.totalLength(List.of())).isZero();
    }
}
