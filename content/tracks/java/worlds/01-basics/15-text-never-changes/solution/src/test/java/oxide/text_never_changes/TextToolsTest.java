package oxide.text_never_changes;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class TextToolsTest {
    @Test
    void tidyCutsOuterSpaces() {
        assertThat(TextTools.tidy("  Ada ")).isEqualTo("Ada");
        assertThat(TextTools.tidy("Hi there")).isEqualTo("Hi there");
    }

    @Test
    void shoutIsUpperCase() {
        assertThat(TextTools.shout("Ada")).isEqualTo("ADA");
    }

    @Test
    void charCountCountsCharacters() {
        assertThat(TextTools.charCount("Hello")).isEqualTo(5);
        assertThat(TextTools.charCount("")).isEqualTo(0);
    }
}
