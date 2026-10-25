package oxide.cut_a_line;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class LineCutterTest {
    @Test
    void wordsCutsAtSpaces() {
        assertThat(LineCutter.words("to be or not")).containsExactly("to", "be", "or", "not");
        assertThat(LineCutter.words("solo")).containsExactly("solo");
    }

    @Test
    void messageKeepsItsSpaces() {
        assertThat(LineCutter.messageOf("10:15:30 ERROR db Connection refused")).isEqualTo("Connection refused");
        assertThat(LineCutter.messageOf("10:16:11 ERROR web 500 on /login")).isEqualTo("500 on /login");
    }
}
