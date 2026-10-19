package oxide.records;

import java.util.Objects;

/** TODO: turn this class into `public record BlockPos(int x, int y, int z)`. */
public final class BlockPos {
    private final int x;
    private final int y;
    private final int z;

    public BlockPos(int x, int y, int z) {
        // TODO: in the record version this becomes a compact constructor that rejects y outside -64..319
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public int x() {
        return x;
    }

    public int y() {
        return y;
    }

    public int z() {
        return z;
    }

    public BlockPos offset(int dx, int dy, int dz) {
        throw new UnsupportedOperationException("TODO: return a moved copy");
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (!(o instanceof BlockPos other)) {
            return false;
        }
        return x == other.x && y == other.y && z == other.z;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y, z);
    }

    @Override
    public String toString() {
        return "(" + x + ", " + y + ", " + z + ")";
    }
}
