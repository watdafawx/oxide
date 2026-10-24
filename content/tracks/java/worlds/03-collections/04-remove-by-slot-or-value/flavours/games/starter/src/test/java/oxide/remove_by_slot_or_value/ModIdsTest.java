package oxide.remove_by_slot_or_value;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class ModIdsTest {
    @Test
    void dropSlotRemovesThePosition() {
        assertThat(ModIds.dropSlot(List.of(394468, 306612, 238222), 0)).containsExactly(306612, 238222);
        assertThat(ModIds.dropSlot(List.of(394468, 306612, 238222), 2)).containsExactly(394468, 306612);
    }

    @Test
    void dropProjectRemovesTheId() {
        // 1 is a project id here, not position 1
        assertThat(ModIds.dropProject(List.of(1, 3, 7, 9), 1)).containsExactly(3, 7, 9);
        assertThat(ModIds.dropProject(List.of(394468, 306612, 238222), 238222)).containsExactly(394468, 306612);
    }
}
