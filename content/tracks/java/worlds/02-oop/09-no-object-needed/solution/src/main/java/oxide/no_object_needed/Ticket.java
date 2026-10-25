package oxide.no_object_needed;

public class Ticket {
    private final String title;

    private Ticket(String title) {
        this.title = title;
    }

    /** Builds a ticket with a trimmed title. */
    public static Ticket of(String title) {
        return new Ticket(title.trim());
    }

    public String title() {
        return title;
    }
}
