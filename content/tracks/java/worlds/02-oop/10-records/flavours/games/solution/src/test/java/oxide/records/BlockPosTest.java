package oxide.records;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import java.util.HashSet;
import java.util.Set;
import org.junit.jupiter.api.Test;

class BlockPosTest {
    @Test
    void blockPosEqualsByValue() {
        assertThat(BlockPos.class.isRecord()).as("BlockPos should be declared with `record`").isTrue();
        BlockPos a = new BlockPos(10, 64, -3);
        assertThat(a).isEqualTo(new BlockPos(10, 64, -3));
        assertThat(a).isNotEqualTo(new BlockPos(10, 65, -3));
        assertThat(a.hashCode()).isEqualTo(new BlockPos(10, 64, -3).hashCode());
        assertThat(a).hasToString("BlockPos[x=10, y=64, z=-3]");
        Set<BlockPos> set = new HashSet<>(Set.of(a));
        assertThat(set).contains(new BlockPos(10, 64, -3));
    }

    @Test
    void compactConstructorRejectsOutOfWorldY() {
        assertThatThrownBy(() -> new BlockPos(0, 320, 0)).isInstanceOf(IllegalArgumentException.class);
        assertThatThrownBy(() -> new BlockPos(0, -65, 0)).isInstanceOf(IllegalArgumentException.class);
        assertThat(new BlockPos(0, -64, 0).y()).isEqualTo(-64);
        assertThat(new BlockPos(0, 319, 0).y()).isEqualTo(319);
    }

    @Test
    void offsetReturnsMovedCopy() {
        BlockPos start = new BlockPos(10, 64, -3);
        BlockPos moved = start.offset(1, 2, -4);
        assertThat(moved).isEqualTo(new BlockPos(11, 66, -7));
        assertThat(start).as("the original never changes").isEqualTo(new BlockPos(10, 64, -3));
        assertThatThrownBy(() -> start.offset(0, 1000, 0)).isInstanceOf(IllegalArgumentException.class);
    }
}
