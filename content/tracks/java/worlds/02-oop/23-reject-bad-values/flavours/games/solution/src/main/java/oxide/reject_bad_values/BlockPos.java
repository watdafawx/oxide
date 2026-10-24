package oxide.reject_bad_values;

public record BlockPos(int x, int y, int z) {
    public BlockPos {
        if (y < -64 || y > 319) {
            throw new IllegalArgumentException("y is outside the world: " + y);
        }
        if (x < -30_000_000 || x > 30_000_000 || z < -30_000_000 || z > 30_000_000) {
            throw new IllegalArgumentException("beyond the world border: " + x + ", " + z);
        }
    }
}
