package oxide.the_one_line_choice;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusWordsTest {
    @Test
    void statusWordFollowsTheFlag() {
        assertThat(StatusWords.statusWord(true)).isEqualTo("ONLINE");
        assertThat(StatusWords.statusWord(false)).isEqualTo("OFFLINE");
    }

    @Test
    void serverWordFollowsTheCount() {
        assertThat(StatusWords.serverWord(1)).isEqualTo("server");
        assertThat(StatusWords.serverWord(0)).isEqualTo("servers");
        assertThat(StatusWords.serverWord(3)).isEqualTo("servers");
    }
}
