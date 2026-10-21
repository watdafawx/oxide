package oxide.while_loop;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class HalvingTest {
    @Test
    void countsHalvingSteps() {
        assertThat(Halving.halvings(800, 100)).isEqualTo(3);
        assertThat(Halving.halvings(1000, 100)).isEqualTo(4);
    }

    @Test
    void alreadyUnderNeedsNoSteps() {
        assertThat(Halving.halvings(50, 100)).isEqualTo(0);
        assertThat(Halving.halvings(100, 100)).isEqualTo(0);
    }
}
