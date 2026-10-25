package oxide.change_each_item;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class ScreenTest {
    private final List<Player> ranked = List.of(
            new Player("Linus", 95), new Player("Grace", 82), new Player("Ada", 70), new Player("Bob", 41));

    @Test
    void namesTurnsPlayersIntoNames() {
        assertThat(Screen.names(ranked)).containsExactly("Linus", "Grace", "Ada", "Bob");
    }

    @Test
    void podiumKeepsTheFirstThreeNames() {
        assertThat(Screen.podium(ranked)).containsExactly("Linus", "Grace", "Ada");
        assertThat(Screen.podium(ranked.subList(0, 2))).containsExactly("Linus", "Grace");
    }
}
