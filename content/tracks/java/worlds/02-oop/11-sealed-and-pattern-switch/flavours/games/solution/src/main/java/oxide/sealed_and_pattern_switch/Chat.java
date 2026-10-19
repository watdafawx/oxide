package oxide.sealed_and_pattern_switch;

public class Chat {
    /** The chat line for an event. A switch over the sealed interface, with no default branch. */
    public static String line(Event event) {
        return switch (event) {
            case PlayerJoined e -> e.player() + " joined the game";
            case BlockBroken e -> e.player() + " broke " + e.block();
            case ItemDropped e when e.count() == 0 -> "nothing dropped";
            case ItemDropped e -> e.count() + " x " + e.item() + " dropped";
        };
    }

    /** "x y z" for a BlockBroken, "nowhere" for any other event. Use a nested record pattern. */
    public static String where(Event event) {
        return switch (event) {
            case BlockBroken(String player, String block, BlockPos(int x, int y, int z)) -> x + " " + y + " " + z;
            default -> "nowhere";
        };
    }
}
