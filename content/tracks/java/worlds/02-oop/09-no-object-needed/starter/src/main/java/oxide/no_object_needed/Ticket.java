package oxide.no_object_needed;

public class Ticket {
    private final String title;

    // TODO: make this constructor private, so of(...) is the only way in
    public Ticket(String title) {
        this.title = title;
    }

    /** Builds a ticket with a trimmed title. */
    public static Ticket of(String title) {
        throw new UnsupportedOperationException("TODO: build a new Ticket from the trimmed title and hand it back");
    }

    public String title() {
        return title;
    }
}
