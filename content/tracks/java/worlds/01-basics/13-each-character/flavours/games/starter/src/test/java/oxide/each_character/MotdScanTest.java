package oxide.each_character;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class MotdScanTest {
    @Test
    void countsColorCodesInMotd() {
        assertThat(MotdScan.countColorCodes("§aWelcome §lto §rMC")).isEqualTo(3);
        assertThat(MotdScan.countColorCodes("A plain MOTD")).isEqualTo(0);
        assertThat(MotdScan.countColorCodes("")).isEqualTo(0);
    }
}
