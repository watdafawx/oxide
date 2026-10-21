package oxide.fill_the_slots;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusLinesTest {
    @Test
    void statusLineHasThreeSlots() {
        assertThat(StatusLines.statusLine("play.example.net", 3, 20)).isEqualTo("play.example.net: 3/20 online");
        assertThat(StatusLines.statusLine("mc.example.org", 0, 50)).isEqualTo("mc.example.org: 0/50 online");
    }

    @Test
    void onlineLineHasTwoSlots() {
        assertThat(StatusLines.onlineLine("survival", 12)).isEqualTo("survival: 12 online");
    }
}
