package oxide.change_means_copy;

public record BlockPos(int x, int y, int z) {
    public BlockPos {
        if (y < -64 || y > 319) {
            throw new IllegalArgumentException("y is outside the world: " + y);
        }
        if (x < -30_000_000 || x > 30_000_000 || z < -30_000_000 || z > 30_000_000) {
            throw new IllegalArgumentException("beyond the world border: " + x + ", " + z);
        }
    }

    /** The position moved by dx, dy, dz, as a new BlockPos. This one never changes. */
    public BlockPos offset(int dx, int dy, int dz) {
        throw new UnsupportedOperationException("TODO: build a new BlockPos at x + dx, y + dy, z + dz");
    }
}
