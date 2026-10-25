package oxide.equal_by_value;

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
}
