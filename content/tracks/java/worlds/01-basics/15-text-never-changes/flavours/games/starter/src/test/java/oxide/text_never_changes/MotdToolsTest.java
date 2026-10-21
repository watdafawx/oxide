package oxide.text_never_changes;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class MotdToolsTest {
    @Test
    void tidyCutsOuterSpaces() {
        assertThat(MotdTools.tidy("  Welcome ")).isEqualTo("Welcome");
        assertThat(MotdTools.tidy("Hi there")).isEqualTo("Hi there");
    }

    @Test
    void shoutIsUpperCase() {
        assertThat(MotdTools.shout("Welcome")).isEqualTo("WELCOME");
    }

    @Test
    void motdLengthCountsCharacters() {
        assertThat(MotdTools.motdLength("Welcome")).isEqualTo(7);
        assertThat(MotdTools.motdLength("")).isEqualTo(0);
    }
}
