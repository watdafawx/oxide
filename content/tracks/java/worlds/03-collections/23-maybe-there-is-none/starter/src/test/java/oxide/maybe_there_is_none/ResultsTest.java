package oxide.maybe_there_is_none;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class ResultsTest {
    private static final Player ADA = new Player("Ada", 70);
    private static final Player LINUS = new Player("Linus", 95);

    @Test
    void bestPlayerIsEmptyForNoPlayers() {
        assertThat(Results.bestPlayer(List.of(ADA, LINUS))).hasValue(LINUS);
        assertThat(Results.bestPlayer(List.of())).isEmpty();
    }

    @Test
    void winnerNameFallsBackToNobody() {
        assertThat(Results.winnerName(List.of(LINUS, ADA))).isEqualTo("Linus");
        assertThat(Results.winnerName(List.of())).isEqualTo("nobody");
    }
}
