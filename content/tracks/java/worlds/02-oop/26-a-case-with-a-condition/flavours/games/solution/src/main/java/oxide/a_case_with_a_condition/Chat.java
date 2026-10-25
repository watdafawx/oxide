package oxide.a_case_with_a_condition;

public class Chat {
    /** The chat line for any event; a drop of 0 items is "nothing dropped". */
    public static String line(Event event) {
        return switch (event) {
            case PlayerJoined e -> e.player() + " joined the game";
            case BlockBroken e -> e.player() + " broke " + e.block();
            case ItemDropped e when e.count() == 0 -> "nothing dropped";
            case ItemDropped e -> e.count() + " x " + e.item() + " dropped";
        };
    }
}
