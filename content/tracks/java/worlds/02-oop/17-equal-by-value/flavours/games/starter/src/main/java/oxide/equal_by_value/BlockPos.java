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
        // TODO: false unless o is a BlockPos; then equal when x, y and z all match
        return true;
    }
}
