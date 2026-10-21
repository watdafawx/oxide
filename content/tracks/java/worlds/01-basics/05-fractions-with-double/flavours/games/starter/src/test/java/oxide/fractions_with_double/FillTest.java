package oxide.fractions_with_double;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;
import org.junit.jupiter.api.Test;

class FillTest {
    @Test
    void fillRatioKeepsTheFraction() {
        assertThat(Fill.fillRatio(17, 20)).isCloseTo(0.85, within(1e-9));
        assertThat(Fill.fillRatio(0, 20)).isCloseTo(0.0, within(1e-9));
    }

    @Test
    void averagePingKeepsTheHalf() {
        assertThat(Fill.averagePing(30, 45)).isCloseTo(37.5, within(1e-9));
    }
}
