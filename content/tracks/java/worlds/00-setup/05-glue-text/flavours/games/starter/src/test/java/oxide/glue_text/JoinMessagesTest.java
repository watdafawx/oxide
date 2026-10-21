package oxide.glue_text;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class JoinMessagesTest {
    @Test
    void announcesOnePlayer() {
        assertThat(JoinMessages.announceJoin("Ada")).isEqualTo("Ada joined the game");
        assertThat(JoinMessages.announceJoin("Bo")).isEqualTo("Bo joined the game");
    }

    @Test
    void announcesTwoPlayers() {
        assertThat(JoinMessages.announcePair("Ada", "Bo")).isEqualTo("Ada and Bo joined the game");
    }
}
