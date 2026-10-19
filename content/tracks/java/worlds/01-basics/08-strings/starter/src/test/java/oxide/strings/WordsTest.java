package oxide.strings;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class WordsTest {
    @Test
    void reversesText() {
        assertThat(Words.reverse("Java")).isEqualTo("avaJ");
        assertThat(Words.reverse("")).isEqualTo("");
    }

    @Test
    void palindromeIgnoresCaseAndSpaces() {
        assertThat(Words.isPalindrome("Never odd or even")).isTrue();
        assertThat(Words.isPalindrome("A man a plan a canal Panama")).isTrue();
        assertThat(Words.isPalindrome("Hello")).isFalse();
    }

    @Test
    void equalsComparesContent() {
        String fresh = new String("java");
        assertThat(Words.sameText(fresh, "java")).isTrue();
        assertThat(Words.sameText("Java", "java")).isFalse();
    }
}
