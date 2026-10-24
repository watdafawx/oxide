package oxide.static_members;

public class Mob {
    private static int nextId = 1;

    private final int id;
    private final String kind;

    private Mob(String kind) {
        this.kind = kind;
        this.id = 0; // TODO: take nextId, then move it forward
    }

    /** Spawns a mob. The constructor is private, so this is the only way. */
    public static Mob spawn(String kind) {
        throw new UnsupportedOperationException("TODO: build a new Mob of that kind and return it");
    }

    public int id() {
        return id;
    }

    public String kind() {
        return kind;
    }
}
