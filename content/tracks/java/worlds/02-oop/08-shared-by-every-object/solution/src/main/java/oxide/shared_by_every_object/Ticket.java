package oxide.shared_by_every_object;

public class Ticket {
    private static int nextId = 1;

    private final int id;
    private final String title;

    public Ticket(String title) {
        this.title = title;
        this.id = nextId;
        nextId++;
    }

    public int id() {
        return id;
    }

    public String title() {
        return title;
    }
}
