package oxide.if_and_else;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BranchingTest {
    @Test
    void clampKeepsInRange() {
        assertThat(Branching.clamp(5, 0, 10)).isEqualTo(5);
        assertThat(Branching.clamp(-3, 0, 10)).isEqualTo(0);
        assertThat(Branching.clamp(99, 0, 10)).isEqualTo(10);
        assertThat(Branching.clamp(10, 0, 10)).isEqualTo(10);
    }

    @Test
    void signOfReturnsMinusOneZeroOne() {
        assertThat(Branching.signOf(-9)).isEqualTo(-1);
        assertThat(Branching.signOf(0)).isEqualTo(0);
        assertThat(Branching.signOf(42)).isEqualTo(1);
    }
}
