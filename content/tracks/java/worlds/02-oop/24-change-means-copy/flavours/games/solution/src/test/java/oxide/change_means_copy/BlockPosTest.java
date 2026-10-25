package oxide.change_means_copy;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class BlockPosTest {
    @Test
    void offsetReturnsMovedCopy() {
        BlockPos chest = new BlockPos(10, 64, -3);
        BlockPos above = chest.offset(0, 1, 0);
        assertThat(above).isEqualTo(new BlockPos(10, 65, -3));
        assertThat(chest.offset(-2, 0, 5)).isEqualTo(new BlockPos(8, 64, 2));
        assertThat(chest.y()).as("the original never moves").isEqualTo(64);
    }

    @Test
    void offsetFollowsTheSameRules() {
        BlockPos top = new BlockPos(0, 319, 0);
        assertThatThrownBy(() -> top.offset(0, 1, 0)).isInstanceOf(IllegalArgumentException.class);
    }
}
