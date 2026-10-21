package oxide.counted_loop;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusLoopsTest {
    @Test
    void playerMinutesAddsInclusive() {
        assertThat(StatusLoops.playerMinutes(10)).isEqualTo(55);
        assertThat(StatusLoops.playerMinutes(1)).isEqualTo(1);
        assertThat(StatusLoops.playerMinutes(0)).isEqualTo(0);
    }

    @Test
    void playerBarHasOneHashPerPlayer() {
        assertThat(StatusLoops.playerBar(3)).isEqualTo("###");
        assertThat(StatusLoops.playerBar(0)).isEqualTo("");
    }
}
