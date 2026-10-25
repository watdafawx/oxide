package oxide.shared_by_every_object;

public class Mob {
    private static int nextId = 1;

    private final int id;
    private final String kind;

    public Mob(String kind) {
        this.kind = kind;
        this.id = nextId;
        nextId++;
    }

    public int id() {
        return id;
    }

    public String kind() {
        return kind;
    }
}
