package oxide.if_and_else;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class PlayerCountTest {
    @Test
    void clampKeepsPlayerCountInRange() {
        assertThat(PlayerCount.clampPlayerCount(5, 0, 20)).isEqualTo(5);
        assertThat(PlayerCount.clampPlayerCount(-1, 0, 20)).isEqualTo(0);
        assertThat(PlayerCount.clampPlayerCount(300, 0, 20)).isEqualTo(20);
        assertThat(PlayerCount.clampPlayerCount(20, 0, 20)).isEqualTo(20);
    }

    @Test
    void trendReturnsMinusOneZeroOne() {
        assertThat(PlayerCount.trend(-4)).isEqualTo(-1);
        assertThat(PlayerCount.trend(0)).isEqualTo(0);
        assertThat(PlayerCount.trend(7)).isEqualTo(1);
    }
}
