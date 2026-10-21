package oxide.equals_not_double_equals;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class TextCompareTest {
    @Test
    void sameTextComparesContent() {
        String fresh = new String("java");
        assertThat(TextCompare.sameText(fresh, "java")).isTrue();
        assertThat(TextCompare.sameText("Java", "java")).isFalse();
    }

    @Test
    void sameIgnoringCaseSkipsCapitals() {
        assertThat(TextCompare.sameIgnoringCase("Java", "JAVA")).isTrue();
        assertThat(TextCompare.sameIgnoringCase("Java", "Lua")).isFalse();
    }
}
