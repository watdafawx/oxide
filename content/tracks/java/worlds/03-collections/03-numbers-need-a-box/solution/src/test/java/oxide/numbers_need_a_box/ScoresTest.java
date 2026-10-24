package oxide.numbers_need_a_box;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class ScoresTest {
    @Test
    void totalAddsEveryScore() {
        assertThat(Scores.total(List.of(70, 95, 82))).isEqualTo(247);
        assertThat(Scores.total(List.of(5))).isEqualTo(5);
    }

    @Test
    void curvedAddsBonusToEach() {
        assertThat(Scores.curved(List.of(70, 95, 82), 5)).containsExactly(75, 100, 87);
        assertThat(Scores.curved(List.of(), 5)).isEmpty();
    }
}
