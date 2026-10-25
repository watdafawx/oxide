package oxide.split_in_two;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class ExamTest {
    @Test
    void passFailSplitsAtFifty() {
        Map<Boolean, List<Integer>> piles = Exam.passFail(List.of(80, 49, 50, 12, 99));
        assertThat(piles.get(true)).containsExactly(80, 50, 99);
        assertThat(piles.get(false)).containsExactly(49, 12);
        assertThat(Exam.passFail(List.of(70)).get(false)).isEmpty();
    }
}
