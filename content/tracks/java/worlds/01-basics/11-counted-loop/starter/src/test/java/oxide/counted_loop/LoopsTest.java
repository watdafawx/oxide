package oxide.counted_loop;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class LoopsTest {
    @Test
    void sumToAddsInclusive() {
        assertThat(Loops.sumTo(10)).isEqualTo(55);
        assertThat(Loops.sumTo(1)).isEqualTo(1);
        assertThat(Loops.sumTo(0)).isEqualTo(0);
    }

    @Test
    void starsMakesOneStarPerRound() {
        assertThat(Loops.stars(3)).isEqualTo("***");
        assertThat(Loops.stars(0)).isEqualTo("");
    }
}
