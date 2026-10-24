package oxide.sealed_and_pattern_switch;

public class Chat {
    /** The chat line for an event. A switch over the sealed interface, with no default branch. */
    public static String line(Event event) {
        throw new UnsupportedOperationException("TODO: switch (event) with one case per record, plus a `when` guard");
    }

    /** "x y z" for a BlockBroken, "nowhere" for any other event. Use a nested record pattern. */
    public static String where(Event event) {
        throw new UnsupportedOperationException("TODO: switch with a nested record pattern that unpacks a BlockBroken and the BlockPos inside it");
    }
}
