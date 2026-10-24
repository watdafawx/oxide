package oxide.reject_bad_values;

public record BlockPos(int x, int y, int z) {
    // TODO: a compact constructor that throws IllegalArgumentException when y is outside -64..319,
    //       and when x or z is beyond the world border (more than 30000000 either way)
}
