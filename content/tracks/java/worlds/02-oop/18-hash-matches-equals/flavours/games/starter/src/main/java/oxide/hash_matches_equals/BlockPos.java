package oxide.hash_matches_equals;

import java.util.Objects;

public class BlockPos {
    private final int x;
    private final int y;
    private final int z;

    public BlockPos(int x, int y, int z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof BlockPos other)) {
            return false;
        }
        return x == other.x && y == other.y && z == other.z;
    }

    @Override
    public int hashCode() {
        // TODO: use the same fields as equals, with Objects.hash(...)
        return System.identityHashCode(this);
    }
}
