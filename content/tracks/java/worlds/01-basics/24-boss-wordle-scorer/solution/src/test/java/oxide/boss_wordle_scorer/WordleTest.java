package oxide.boss_wordle_scorer;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class WordleTest {
    @Test
    void allGreenOnExactMatch() {
        assertThat(Wordle.score("crane", "crane")).isEqualTo("GGGGG");
        assertThat(Wordle.score("toast", "toast")).isEqualTo("GGGGG");
    }

    @Test
    void yellowForWrongPosition() {
        assertThat(Wordle.score("crane", "nacre")).isEqualTo("YYYYG");
        assertThat(Wordle.score("crane", "cloud")).isEqualTo("G....");
        assertThat(Wordle.score("crane", "slimy")).isEqualTo(".....");
    }

    @Test
    void duplicateLettersScoredOnce() {
        assertThat(Wordle.score("crane", "eerie")).isEqualTo("..Y.G");
        assertThat(Wordle.score("abbey", "babes")).isEqualTo("YYGG.");
    }

    @Test
    void rendersEmojiRow() {
        String green = "🟩";
        String yellow = "🟨";
        String gray = "⬛";
        assertThat(Wordle.render("GY.")).isEqualTo(green + yellow + gray);
        assertThat(Wordle.render(Wordle.score("crane", "crane"))).isEqualTo(green.repeat(5));
    }
}
