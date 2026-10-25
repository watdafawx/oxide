package oxide.the_compiler_counts;

public class Chat {
    /** The chat line for any event. A switch over the sealed interface, with no default. */
    public static String line(Event event) {
        throw new UnsupportedOperationException("TODO: switch (event) with one case per record: \"Alex joined the game\", \"Alex broke stone\", \"3 x diamond dropped\"");
    }
}
