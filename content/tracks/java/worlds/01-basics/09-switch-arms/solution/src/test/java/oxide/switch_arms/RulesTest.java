package oxide.switch_arms;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import org.junit.jupiter.api.Test;

class RulesTest {
    @Test
    void gradeForMapsBands() {
        assertThat(Rules.gradeFor(100)).isEqualTo("A");
        assertThat(Rules.gradeFor(90)).isEqualTo("A");
        assertThat(Rules.gradeFor(89)).isEqualTo("B");
        assertThat(Rules.gradeFor(72)).isEqualTo("C");
        assertThat(Rules.gradeFor(65)).isEqualTo("D");
        assertThat(Rules.gradeFor(59)).isEqualTo("F");
        assertThat(Rules.gradeFor(0)).isEqualTo("F");
    }

    @Test
    void daysInMonthHandlesLeapYear() {
        assertThat(Rules.daysInMonth(1, 2025)).isEqualTo(31);
        assertThat(Rules.daysInMonth(4, 2025)).isEqualTo(30);
        assertThat(Rules.daysInMonth(12, 2025)).isEqualTo(31);
        assertThat(Rules.daysInMonth(2, 2025)).isEqualTo(28);
        assertThat(Rules.daysInMonth(2, 2024)).isEqualTo(29);
        assertThat(Rules.daysInMonth(2, 1900)).isEqualTo(28);
        assertThat(Rules.daysInMonth(2, 2000)).isEqualTo(29);
        assertThatThrownBy(() -> Rules.daysInMonth(13, 2025)).isInstanceOf(IllegalArgumentException.class);
    }
}
