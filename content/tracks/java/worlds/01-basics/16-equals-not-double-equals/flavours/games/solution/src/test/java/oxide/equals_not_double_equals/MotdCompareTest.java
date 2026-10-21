package oxide.equals_not_double_equals;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class MotdCompareTest {
    @Test
    void sameMotdComparesContent() {
        String polled = new String("Welcome to Hopper SMP");
        assertThat(MotdCompare.sameMotd(polled, "Welcome to Hopper SMP")).isTrue();
        assertThat(MotdCompare.sameMotd("Welcome", "welcome")).isFalse();
    }

    @Test
    void sameIgnoringCaseSkipsCapitals() {
        assertThat(MotdCompare.sameIgnoringCase("Welcome", "WELCOME")).isTrue();
        assertThat(MotdCompare.sameIgnoringCase("Welcome", "Goodbye")).isFalse();
    }
}
