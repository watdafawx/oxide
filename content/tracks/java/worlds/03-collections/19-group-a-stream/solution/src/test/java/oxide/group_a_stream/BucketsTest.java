package oxide.group_a_stream;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class BucketsTest {
    private final List<String> words = List.of("cat", "horse", "dog", "mouse", "ox");

    @Test
    void groupByLengthBucketsWords() {
        assertThat(Buckets.groupByLength(words)).hasSize(3)
                .containsEntry(3, List.of("cat", "dog"))
                .containsEntry(5, List.of("horse", "mouse"))
                .containsEntry(2, List.of("ox"));
    }

    @Test
    void countByLengthCountsEachBucket() {
        assertThat(Buckets.countByLength(words)).hasSize(3)
                .containsEntry(3, 2L).containsEntry(5, 2L).containsEntry(2, 1L);
    }
}
