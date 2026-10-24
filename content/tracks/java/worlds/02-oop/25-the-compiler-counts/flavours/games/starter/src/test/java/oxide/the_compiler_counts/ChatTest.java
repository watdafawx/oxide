package oxide.the_compiler_counts;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ChatTest {
    @Test
    void eventIsSealed() {
        assertThat(Event.class.isSealed()).as("write `sealed interface Event permits ...`").isTrue();
        assertThat(Event.class.getPermittedSubclasses()).containsExactlyInAnyOrder(PlayerJoined.class, BlockBroken.class, ItemDropped.class);
    }

    @Test
    void chatLineForEveryEventType() {
        assertThat(Chat.line(new PlayerJoined("Alex"))).isEqualTo("Alex joined the game");
        assertThat(Chat.line(new BlockBroken("Alex", "stone", new BlockPos(1, 64, 2)))).isEqualTo("Alex broke stone");
        assertThat(Chat.line(new ItemDropped("diamond", 3))).isEqualTo("3 x diamond dropped");
    }
}
