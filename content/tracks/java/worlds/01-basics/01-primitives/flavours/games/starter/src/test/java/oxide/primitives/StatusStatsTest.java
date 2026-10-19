package oxide.primitives;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusStatsTest {
    @Test
    void playerCountWrapsPastIntMax() {
        assertThat(StatusStats.addPlayers(20, 3)).isEqualTo(23);
        assertThat(StatusStats.addPlayers(Integer.MAX_VALUE, 1)).isEqualTo(Integer.MIN_VALUE);
    }

    @Test
    void playerSecondsFitInLong() {
        assertThat(StatusStats.playerSeconds(100_000, 100_000)).isEqualTo(10_000_000_000L);
        assertThat(StatusStats.playerSeconds(20, 60)).isEqualTo(1_200L);
    }
}
