package oxide.equals_hashcode_tostring;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import org.junit.jupiter.api.Test;

class BlockPosTest {
    @Test
    void samePositionIsEqual() {
        BlockPos a = new BlockPos(10, 64, -3);
        assertThat(a).isEqualTo(new BlockPos(10, 64, -3));
        assertThat(a).isNotEqualTo(new BlockPos(11, 64, -3));
        assertThat(a).isNotEqualTo(new BlockPos(10, 65, -3));
        assertThat(a).isNotEqualTo(new BlockPos(10, 64, 3));
        assertThat(a).isNotEqualTo(null);
        assertThat(a).isNotEqualTo("(10, 64, -3)");
    }

    @Test
    void hashSetFindsEqualPosition() {
        BlockPos a = new BlockPos(10, 64, -3);
        BlockPos b = new BlockPos(10, 64, -3);
        assertThat(a).isEqualTo(b);
        assertThat(a.hashCode()).isEqualTo(b.hashCode());

        Set<BlockPos> torches = new HashSet<>();
        torches.add(a);
        assertThat(torches).contains(b);
        torches.add(b);
        assertThat(torches).as("a set holds a position once").hasSize(1);

        Map<BlockPos, String> chests = new HashMap<>();
        chests.put(a, "storage");
        assertThat(chests.get(b)).isEqualTo("storage");
    }

    @Test
    void toStringReadsLikeAChatCoordinate() {
        assertThat(new BlockPos(10, 64, -3)).hasToString("(10, 64, -3)");
        assertThat(new BlockPos(0, 0, 0)).hasToString("(0, 0, 0)");
    }
}
