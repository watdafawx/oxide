package oxide.shared_by_every_object;

public class Mob {
    // TODO (after the constructor): this counter must be shared by every mob
    private int nextId = 1;

    private final int id;
    private final String kind;

    public Mob(String kind) {
        this.kind = kind;
        // TODO: take the id from nextId, then move nextId on by one
        this.id = 0;
    }

    public int id() {
        return id;
    }

    public String kind() {
        return kind;
    }
}
