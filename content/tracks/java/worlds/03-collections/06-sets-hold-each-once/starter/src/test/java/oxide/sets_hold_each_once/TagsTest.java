package oxide.sets_hold_each_once;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class TagsTest {
    @Test
    void distinctDropsRepeats() {
        assertThat(Tags.distinct(List.of("java", "code", "java", "tips", "code")))
                .containsExactlyInAnyOrder("java", "code", "tips");
    }

    @Test
    void firstRepeatFindsTheEarliestDuplicate() {
        assertThat(Tags.firstRepeat(List.of("java", "code", "tips", "code", "java"))).isEqualTo("code");
        assertThat(Tags.firstRepeat(List.of("a", "b", "c"))).isNull();
        assertThat(Tags.firstRepeat(List.of("x", "x"))).isEqualTo("x");
    }
}
