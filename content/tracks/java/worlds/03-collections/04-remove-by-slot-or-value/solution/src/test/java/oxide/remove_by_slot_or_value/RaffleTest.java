package oxide.remove_by_slot_or_value;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class RaffleTest {
    @Test
    void dropSlotRemovesThePosition() {
        assertThat(Raffle.dropSlot(List.of(512, 7, 33), 0)).containsExactly(7, 33);
        assertThat(Raffle.dropSlot(List.of(512, 7, 33), 2)).containsExactly(512, 7);
    }

    @Test
    void dropNumberRemovesTheValue() {
        // 1 is a ticket number here, not position 1
        assertThat(Raffle.dropNumber(List.of(1, 3, 7, 9), 1)).containsExactly(3, 7, 9);
        assertThat(Raffle.dropNumber(List.of(512, 7, 33), 33)).containsExactly(512, 7);
    }
}
