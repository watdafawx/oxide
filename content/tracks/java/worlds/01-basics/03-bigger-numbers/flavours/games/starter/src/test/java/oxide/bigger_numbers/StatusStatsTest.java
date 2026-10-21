package oxide.bigger_numbers;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusStatsTest {
    @Test
    void bytesSentNeedsLongLiteral() {
        assertThat(StatusStats.bytesSentLastYear()).isEqualTo(6_000_000_000L);
    }

    @Test
    void playerSecondsFitInLong() {
        assertThat(StatusStats.playerSeconds(100_000, 100_000)).isEqualTo(10_000_000_000L);
        assertThat(StatusStats.playerSeconds(20, 60)).isEqualTo(1_200L);
    }
}
