package oxide.boss_first_report;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BadgeTest {
    @Test
    void titleWelcomesTheEvent() {
        assertThat(Badge.title("DevDay")).isEqualTo("Welcome to DevDay");
    }

    @Test
    void seatLineHasThreeValues() {
        assertThat(Badge.seatLine("Ada", 3, 40)).isEqualTo("Ada: seat 3 of 40");
    }

    @Test
    void eventComesFromArgs() {
        assertThat(Badge.eventOf(new String[] {"DevDay"})).isEqualTo("DevDay");
    }

    @Test
    void badgeHasTwoLines() {
        assertThat(Badge.badge(new String[] {"DevDay"}, "Ada", 3, 40)).isEqualTo("Welcome to DevDay\nAda: seat 3 of 40");
    }
}
