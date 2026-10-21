package oxide.flip_it_round;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class MotdTest {
    @Test
    void flipsMotdBackwards() {
        assertThat(Motd.flip("Welcome")).isEqualTo("emocleW");
        assertThat(Motd.flip("")).isEqualTo("");
    }

    @Test
    void palindromeMotdIgnoresCaseAndSpaces() {
        assertThat(Motd.isPalindrome("Was it a car or a cat I saw")).isTrue();
        assertThat(Motd.isPalindrome("Never odd or even")).isTrue();
        assertThat(Motd.isPalindrome("Welcome")).isFalse();
    }
}
