package oxide.loops;

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
    void countsColorCodesInMotd() {
        assertThat(StatusLoops.countColorCodes("§aWelcome §lto §rMC")).isEqualTo(3);
        assertThat(StatusLoops.countColorCodes("A plain MOTD")).isEqualTo(0);
        assertThat(StatusLoops.countColorCodes("")).isEqualTo(0);
    }
}
