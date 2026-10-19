package oxide.records;

public record BlockPos(int x, int y, int z) {
    public BlockPos {
        if (y < -64 || y > 319) {
            throw new IllegalArgumentException("y outside the world: " + y);
        }
    }

    public BlockPos offset(int dx, int dy, int dz) {
        return new BlockPos(x + dx, y + dy, z + dz);
    }
}
