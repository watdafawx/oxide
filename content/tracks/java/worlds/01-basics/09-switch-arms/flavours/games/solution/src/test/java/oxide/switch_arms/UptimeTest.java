package oxide.switch_arms;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import org.junit.jupiter.api.Test;

class UptimeTest {
    @Test
    void pingGradeMapsBands() {
        assertThat(Uptime.pingGrade(0)).isEqualTo("great");
        assertThat(Uptime.pingGrade(49)).isEqualTo("great");
        assertThat(Uptime.pingGrade(50)).isEqualTo("good");
        assertThat(Uptime.pingGrade(99)).isEqualTo("good");
        assertThat(Uptime.pingGrade(100)).isEqualTo("ok");
        assertThat(Uptime.pingGrade(199)).isEqualTo("ok");
        assertThat(Uptime.pingGrade(200)).isEqualTo("poor");
        assertThat(Uptime.pingGrade(900)).isEqualTo("poor");
    }

    @Test
    void monthLengthHandlesLeapYear() {
        assertThat(Uptime.daysInMonth(1, 2025)).isEqualTo(31);
        assertThat(Uptime.daysInMonth(4, 2025)).isEqualTo(30);
        assertThat(Uptime.daysInMonth(12, 2025)).isEqualTo(31);
        assertThat(Uptime.daysInMonth(2, 2025)).isEqualTo(28);
        assertThat(Uptime.daysInMonth(2, 2024)).isEqualTo(29);
        assertThat(Uptime.daysInMonth(2, 1900)).isEqualTo(28);
        assertThat(Uptime.daysInMonth(2, 2000)).isEqualTo(29);
        assertThatThrownBy(() -> Uptime.daysInMonth(13, 2025)).isInstanceOf(IllegalArgumentException.class);
    }
}
