package oxide.fractions_with_double;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;
import org.junit.jupiter.api.Test;

class FractionsTest {
    @Test
    void ratioKeepsTheFraction() {
        assertThat(Fractions.ratio(17, 20)).isCloseTo(0.85, within(1e-9));
        assertThat(Fractions.ratio(0, 20)).isCloseTo(0.0, within(1e-9));
    }

    @Test
    void averageKeepsTheHalf() {
        assertThat(Fractions.average(30, 45)).isCloseTo(37.5, within(1e-9));
    }
}
