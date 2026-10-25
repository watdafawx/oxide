package oxide.the_compiler_counts;

public class Chat {
    /** The chat line for any event. A switch over the sealed interface, with no default. */
    public static String line(Event event) {
        return switch (event) {
            case PlayerJoined e -> e.player() + " joined the game";
            case BlockBroken e -> e.player() + " broke " + e.block();
            case ItemDropped e -> e.count() + " x " + e.item() + " dropped";
        };
    }
}
