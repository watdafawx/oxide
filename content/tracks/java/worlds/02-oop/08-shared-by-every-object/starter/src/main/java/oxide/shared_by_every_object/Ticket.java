package oxide.shared_by_every_object;

public class Ticket {
    // TODO (after the constructor): this counter must be shared by every ticket
    private int nextId = 1;

    private final int id;
    private final String title;

    public Ticket(String title) {
        this.title = title;
        // TODO: take the id from nextId, then move nextId on by one
        this.id = 0;
    }

    public int id() {
        return id;
    }

    public String title() {
        return title;
    }
}
