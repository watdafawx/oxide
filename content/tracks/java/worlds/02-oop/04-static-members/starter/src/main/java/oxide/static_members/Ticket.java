package oxide.static_members;

public class Ticket {
    private static int nextId = 1;

    private final int id;
    private final String title;

    private Ticket(String title) {
        this.title = title;
        this.id = 0; // TODO: take nextId, then move it forward
    }

    /** Builds a ticket. The constructor is private, so this is the only way. */
    public static Ticket of(String title) {
        throw new UnsupportedOperationException("TODO: build a Ticket with new Ticket(title)");
    }

    public int id() {
        return id;
    }

    public String title() {
        return title;
    }
}
