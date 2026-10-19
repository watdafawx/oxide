package oxide.integer_division;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;

import org.junit.jupiter.api.Test;

class DivisionTest {
    @Test
    void dividesWithRemainder() {
        assertThat(Division.divmod(17, 5)).containsExactly(3, 2);
        assertThat(Division.divmod(-7, 2)).containsExactly(-3, -1);
        assertThat(Division.ratio(17, 5)).isCloseTo(3.4, within(1e-9));
    }
}
