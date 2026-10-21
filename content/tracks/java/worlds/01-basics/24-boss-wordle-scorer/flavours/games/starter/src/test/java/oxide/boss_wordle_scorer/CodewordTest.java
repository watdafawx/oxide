package oxide.boss_wordle_scorer;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class CodewordTest {
    @Test
    void exactCodewordIsAllGreen() {
        assertThat(Codeword.score("sword", "sword")).isEqualTo("GGGGG");
        assertThat(Codeword.score("slime", "slime")).isEqualTo("GGGGG");
    }

    @Test
    void misplacedLetterIsYellow() {
        assertThat(Codeword.score("sword", "words")).isEqualTo("YYYYY");
        assertThat(Codeword.score("slime", "smile")).isEqualTo("GYGYG");
        assertThat(Codeword.score("sword", "sworn")).isEqualTo("GGGG.");
    }

    @Test
    void repeatedLetterIsScoredOnce() {
        assertThat(Codeword.score("pearl", "eerie")).isEqualTo(".GY..");
        assertThat(Codeword.score("creep", "eerie")).isEqualTo("YYY..");
    }

    @Test
    void replyRendersEmojiRow() {
        String green = "🟩";
        String yellow = "🟨";
        assertThat(Codeword.reply("Ada", "slime", "smile"))
                .isEqualTo("Ada: " + green + yellow + green + yellow + green);
        assertThat(Codeword.reply("Bo", "sword", "sword")).isEqualTo("Bo: " + green.repeat(5));
    }
}
