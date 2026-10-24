package oxide.unpack_in_the_case;

public class Chat {
    /** "x y z" of a broken block, or "nowhere" for any other event. */
    public static String where(Event event) {
        return switch (event) {
            case BlockBroken(String player, String block, BlockPos(int x, int y, int z)) -> x + " " + y + " " + z;
            case PlayerJoined e -> "nowhere";
            case ItemDropped e -> "nowhere";
        };
    }
}
