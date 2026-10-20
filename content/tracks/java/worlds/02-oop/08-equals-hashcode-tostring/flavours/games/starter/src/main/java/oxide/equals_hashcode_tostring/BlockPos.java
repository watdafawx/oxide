package oxide.equals_hashcode_tostring;

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

    public int x() {
        return x;
    }

    public int y() {
        return y;
    }

    public int z() {
        return z;
    }

    @Override
    public boolean equals(Object o) {
        // TODO: equal when it is a BlockPos with the same x, y and z
        return this == o;
    }

    @Override
    public int hashCode() {
        // TODO: use the same fields as equals, e.g. Objects.hash(...)
        return System.identityHashCode(this);
    }

    @Override
    public String toString() {
        // TODO: "(10, 64, -3)"
        return super.toString();
    }
}
